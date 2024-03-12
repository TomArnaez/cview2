use std::{collections::VecDeque, sync::Arc, time::Duration};
use log::info;
use specta::Type;
use serde::{Deserialize, Serialize};
use tokio::{sync::{mpsc, watch, Mutex}};
use uuid::Uuid;
use wrapper::{ExposureModes, FullWellModes, SLBufferInfo, SLError, SLImage, ROI};
use tauri::async_runtime::JoinHandle;
use super::{detector::DetectorCaptureHandle, error::JobError, report::{CaptureReport, CaptureReportUpdate}};

pub struct Capture<Capture: StatefulCapture> {
    id: Uuid,
    report: Option<CaptureReport>,
    state: Option<CaptureState<Capture>>
}

impl <SJob: StatefulCapture> Capture<SJob> {
    pub fn new(init: SJob) -> Self {
        CaptureBuilder::new(init).build()
    }
}

pub struct CaptureBuilder<SJob: StatefulCapture> {
    id: Uuid,
    init: SJob
}

impl<SJob: StatefulCapture> CaptureBuilder<SJob> {
    pub fn build(self) -> Capture<SJob> {
        Capture::<SJob> {
            id: self.id,
            report: None,
            state: Some(CaptureState {
                data: None,
                init: self.init,
                steps: VecDeque::new(),
                step_number: 0
            })
        }
    }

	pub fn new(init: SJob) -> Self {
		let id = Uuid::new_v4();
		Self {
			id,
			init,
		}
    }
}

pub struct CaptureState<Capture: StatefulCapture> {
    pub init: Capture,
    pub data: Option<Capture::Data>,
    pub steps: VecDeque<Capture::Step>,
    pub step_number: usize,
}

#[async_trait::async_trait]
pub trait StatefulCapture: Send + Sync + 'static {
    type Data: Send + Sync;
    type Step: Send + Sync;
    type Result;

    async fn init(&self, detector_handle: DetectorCaptureHandle) -> Result<JobInitOutput<Self::Step, Self::Data>, JobError>;
    async fn execute_step(&self, step: &Self::Step, data: &mut Self::Data, events_tx: mpsc::Sender<CaptureReportUpdate>);
    async fn finalise(&self, data: Self::Data) -> Self::Result;
}

#[async_trait::async_trait]
pub trait DynCapture: Send + Sync {
    fn id(&self) -> Uuid;
    fn report(&self) -> &Option<CaptureReport>;
    async fn run(&mut self, detector_handle: DetectorCaptureHandle, rx: watch::Receiver<CaptureCommand>, events_tx: mpsc::Sender<CaptureReportUpdate>) -> Result<(), JobError>;
}

#[async_trait::async_trait]
impl<SJob: StatefulCapture> DynCapture for Capture<SJob> {
    fn id(&self) -> Uuid {
        self.id
    }

    fn report(&self) -> &Option<CaptureReport> {
        &self.report
    }

    async fn run(&mut self, detector_handle: DetectorCaptureHandle, rx: watch::Receiver<CaptureCommand>, events_tx: mpsc::Sender<CaptureReportUpdate>) -> Result<(), JobError> {
        let id = self.id();
        info!("Starting capture <id={id}>");

        let CaptureState { init, data, mut steps, mut step_number} = self.state.take().expect("criticla error: missing capture state");

        let stateful_job = Arc::new(init);
        let working_data = Arc::new(Mutex::new(data));

        let init_task = {
            let events_tx = events_tx.clone();
            let stateful_job = Arc::clone(&stateful_job);
            tauri::async_runtime::spawn(async move {
                let res = stateful_job.init(detector_handle).await;

                if let Ok(res) = res.as_ref() {
                    events_tx.send(CaptureReportUpdate::TaskCount(res.steps.len())).await.unwrap();
                }
                res
            })
        };

        let JobInitOutput { mut steps, data } = handle_init_phase::<SJob>(init_task, rx.clone()).await?;
        *working_data.lock().await = Some(data);

        while !steps.is_empty() {
            let stateful_job = Arc::clone(&stateful_job);
            let working_data = Arc::clone(&working_data);
            
            let step = Arc::new(steps.pop_front().unwrap());
            let events_tx = events_tx.clone();
            let step_task = tauri::async_runtime::spawn(async move {
                let mut lock = working_data.lock().await;
                let data = lock.as_mut().unwrap();
                stateful_job.execute_step(&step, data, events_tx).await;
            });

            handle_single_step(step_task, rx.clone()).await?;

            step_number += 1;
        }

        Ok(())        
    }
}

pub struct JobInitOutput<Step, Data> {
    pub data: Data,
    pub steps: VecDeque<Step>,
}

async fn handle_init_phase<SJob: StatefulCapture>
(
    mut init_task: JoinHandle<Result<JobInitOutput<SJob::Step, SJob::Data>, JobError>>,
    mut commands_rx: watch::Receiver<CaptureCommand>
) -> Result<JobInitOutput<SJob::Step, SJob::Data>, JobError> {
    tokio::select! {
        result = &mut init_task => {
            return result.unwrap();
        },
        Ok(_) = commands_rx.changed() => {
            println!("got cancel event init");
            match *commands_rx.borrow() {
                CaptureCommand::Cancel => {
                    init_task.abort();
                    info!("Cancelling Job");
                    return Err(JobError::Cancelled);
                },            
            }
        }
    }
}

async fn handle_single_step(
    mut step_task: JoinHandle<()>,
    mut commands_rx: watch::Receiver<CaptureCommand>
) -> Result<(), JobError> {
    tokio::select! {
        result = &mut step_task => {
            return Ok(());
        },
        Ok(_) = commands_rx.changed() => {
            println!("got cancel event step");
            match *commands_rx.borrow() {
                CaptureCommand::Cancel => {
                    step_task.abort();
                    info!("Cancelling Job");
                    return Err(JobError::Cancelled);
                },
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum CaptureType {
    Stream(StreamCapture),
}

#[derive(Debug)]
pub enum CaptureCommand {
    Cancel
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
pub struct CaptureSettings {
    pub dds: bool,
    pub full_well_mode: FullWellModes,
    pub roi: ROI,
    pub test_mode: bool,
    pub timeout: Duration,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct StreamCapture {
    capture_settings: CaptureSettings,
    stream_time: Option<Duration>,
}