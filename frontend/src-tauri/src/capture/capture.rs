use super::{
    capture_modes::CaptureContext, error::CaptureError, report::{CaptureReport, CaptureReportBuilder, CaptureReportUpdate}
};
use log::info;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{collections::VecDeque, sync::Arc, time::Duration};
use tauri::async_runtime::JoinHandle;
use tokio::sync::{watch, Mutex};
use uuid::Uuid;
use wrapper::{FullWellModes, ROI};

pub struct Capture<Capture: StatefulCapture> {
    id: Uuid,
    report: CaptureReport,
    state: Option<CaptureState<Capture>>,
}

impl<SJob: StatefulCapture> Capture<SJob> {
    pub fn new(init: SJob) -> Self {
        CaptureBuilder::new(init).build()
    }
}

pub struct CaptureBuilder<SJob: StatefulCapture> {
    id: Uuid,
    init: SJob,
    report_builder: CaptureReportBuilder,
}

impl<SJob: StatefulCapture> CaptureBuilder<SJob> {
    pub fn build(self) -> Capture<SJob> {
        Capture::<SJob> {
            id: self.id,
            report: self.report_builder.build(),
            state: Some(CaptureState {
                data: None,
                init: self.init,
                steps: VecDeque::new(),
                step_number: 0,
            }),
        }
    }

    pub fn new(init: SJob) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            init,
            report_builder: CaptureReportBuilder::new(id, SJob::NAME.to_string()),
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

    const NAME: &'static str;

    async fn init(
        &self,
        ctx: &CaptureContext
    ) -> Result<JobInitOutput<Self::Step, Self::Data>, CaptureError>;
    async fn execute_step(
        &self,
        step: &Self::Step,
        data: &mut Self::Data,
        ctx: &CaptureContext,
    ) -> Result<CaptureStepOutput, CaptureError>;
    async fn finalise(
        &self,
        data: Self::Data,
        capture_ctx: &CaptureContext
    ) -> Self::Result;
}

impl<SJob: StatefulCapture> Capture<SJob> {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn report(&self) -> &CaptureReport {
        &self.report
    }

    pub fn report_mut(&mut self) -> &mut CaptureReport {
        &mut self.report
    }

    pub async fn run(
        &mut self,
        ctx: CaptureContext,
        rx: watch::Receiver<CaptureCommand>,
    ) -> Result<SJob::Result, CaptureError> {
        let id = self.id();
        info!("Starting capture <id={id}>");

        let CaptureState {
            init,
            data,
            mut steps,
            mut step_number,
        } = self
            .state
            .take()
            .expect("critical error: missing capture state");

        let stateful_job = Arc::new(init);
        let working_data = Arc::new(Mutex::new(data));

        let ctx = Arc::new(ctx);

        let init_task = {
            let ctx = Arc::clone(&ctx);
            let stateful_job = Arc::clone(&stateful_job);
            tauri::async_runtime::spawn(async move {
                let res = stateful_job.init(&ctx).await;

                if let Ok(res) = res.as_ref() {
                    ctx.events_tx
                        .send(CaptureReportUpdate::TaskCount(res.steps.len()))
                        .await
                        .unwrap();
                }
                res
            })
        };

        let JobInitOutput { mut steps, data } =
            handle_init_phase::<SJob>(init_task, rx.clone()).await?;
        *working_data.lock().await = Some(data);

        while !steps.is_empty() {
            let stateful_job = Arc::clone(&stateful_job);
            let working_data = Arc::clone(&working_data);

            let step = Arc::new(steps.pop_front().unwrap());
            let ctx = Arc::clone(&ctx);
            let step_task = tauri::async_runtime::spawn(async move {
                let mut lock = working_data.lock().await;
                let data = lock.as_mut().unwrap();
                stateful_job.execute_step(&step, data, &ctx).await;
            });

            handle_single_step(step_task, rx.clone()).await?;

            step_number += 1;
        }

        // Allow finalise to own the data to do whatever with it
        let data = working_data.lock().await.take().unwrap();
        let res = stateful_job.finalise(data, &ctx).await;
        Ok(res)
    }
}

pub struct JobInitOutput<Step, Data> {
    pub data: Data,
    pub steps: VecDeque<Step>,
}

pub struct CaptureStepOutput {
    pub request_input: Option<String>
}

async fn handle_init_phase<SJob: StatefulCapture>(
    mut init_task: JoinHandle<Result<JobInitOutput<SJob::Step, SJob::Data>, CaptureError>>,
    mut commands_rx: watch::Receiver<CaptureCommand>,
) -> Result<JobInitOutput<SJob::Step, SJob::Data>, CaptureError> {
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
                    return Err(CaptureError::Canceled);
                },
            }
        }
    }
}

async fn handle_single_step(
    mut step_task: JoinHandle<()>,
    mut commands_rx: watch::Receiver<CaptureCommand>,
) -> Result<(), CaptureError> {
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
                    return Err(CaptureError::Canceled);
                },
            }
        }
    }
}

#[derive(Debug)]
pub enum CaptureCommand {
    Cancel,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
pub struct CaptureSettings {
    pub dds: bool,
    pub full_well_mode: FullWellModes,
    pub roi: ROI,
    pub test_mode: bool,
    pub timeout: Duration,
}