use std::{collections::VecDeque, sync::Arc, thread::JoinHandle, time::Duration};
use log::info;
use specta::Type;
use serde::{Deserialize, Serialize};
use tokio::{pin, sync::{mpsc, watch}};
use uuid::Uuid;
use wrapper::{ExposureModes, FullWellModes, SLBufferInfo, SLError, SLImage, ROI};
use tokio_stream::{Stream, StreamExt};
use async_stream::stream;
use super::{detector::DetectorCaptureHandle, report::CaptureReportUpdate};

pub struct Capture<Capture: StatefulCapture> {
    id: Uuid,
    report: Option<CaptureReport>,
    state: Option<CaptureState<Capture>>
}

pub struct CaptureState<Capture: StatefulCapture> {
    pub init: Capture,
    pub data: Option<Capture::Data>,
    pub steps: VecDeque<Capture::Step>,
    pub step_number: usize,
}

pub trait StatefulCapture: Send + Sync + 'static {
    type Data: Send + Sync;
    type Step: Send + Sync;
    type CaptureResult;

    async fn run(&self, detector_handle: DetectorCaptureHandle) -> Result<impl Stream<Item = SLImage>, SLError>;
    async fn init(&self) -> Vec<Self::Step>;
    async fn execute_step(&self, step: &Self::Step, data: &Self::Data);
    async fn finalise(&self, data: &Self::Data) -> Self::CaptureResult;
}

pub trait DynCapture: Send + Sync {
    async fn setup(&self, detector_handle: DetectorCaptureHandle, acquisition_settings: CaptureSettings) -> Result<(), SLError> {
        detector_handle.set_dds(acquisition_settings.dds_on).await?;
        detector_handle.set_full_well_mode(acquisition_settings.full_well_mode).await?;
        detector_handle.set_roi(acquisition_settings.roi).await?;
        detector_handle.set_test_mode(acquisition_settings.test_mode).await?;
        
        Ok(())
    }

    fn id(&self) -> Uuid;
    fn report(&self) -> &Option<CaptureReport>;
    async fn run(&mut self, detector_handle: DetectorCaptureHandle, rx: mpsc::Receiver<CaptureCommand>, events_tx: mpsc::Sender<CaptureReportUpdate>);
}

impl<SJob: StatefulCapture> DynCapture for Capture<SJob> {
    fn id(&self) -> Uuid {
        self.id
    }

    fn report(&self) -> &Option<CaptureReport> {
        &self.report
    }

    async fn run(&mut self, detector_handle: DetectorCaptureHandle, rx: mpsc::Receiver<CaptureCommand>, events_tx: mpsc::Sender<CaptureReportUpdate>) {
        let id = self.id();
        info!("Starting capture <id={id}>");

        let CaptureState { init, data, mut steps, mut step_number} = self.state.take().expect("criticla error: missing capture state");

        let stateful_job = Arc::new(init);
        let working_data = Arc::new(data.unwrap());
        let mut job_should_run = true;

        // Init phase
        let init_task = {
            let stateful_job = Arc::clone(&stateful_job);
            tauri::async_runtime::spawn(async move {
                let res = stateful_job.init().await;
                
                events_tx.send(CaptureReportUpdate::TaskCount(res.len())).await.unwrap();
            })
        };

        while job_should_run && steps.is_empty() {
            let stateful_job = Arc::clone(&stateful_job);
            let working_data = Arc::clone(&working_data);

            let step = Arc::new(steps.pop_front().unwrap());
            let step_task = tauri::async_runtime::spawn(async move {
                stateful_job.execute_step(&step, &working_data).await;
            });
        }        
    }
}

async fn handle_single_step<SJob: StatefulCapture>(
    step_task: JoinHandle<()>,
    mut commands_rx: mpsc::Receiver<CaptureCommand>
) {
    
}


#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum CaptureType {
    Stream(StreamCapture),
    Sequence(SequenceCapture),
}

#[derive(Debug)]
pub enum CaptureCommand {
    Cancel
}

#[derive(Debug, Clone, Serialize, Type)]
pub struct CaptureReport {
    capture: CaptureType,
    frame: u32,
    buffer_info: SLBufferInfo
}

#[derive(Debug)]
pub enum CaptureResponse {
    Error(SLError),
    Report(CaptureReport),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
struct CaptureSettings {
    dds_on: bool,
    full_well_mode: FullWellModes,
    roi: ROI,
    test_mode: bool,
    timeout: Duration,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct StreamCapture {
    capture_settings: CaptureSettings,
    stream_time: Option<Duration>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct SequenceCapture {
    acquisition_settings: CaptureSettings,
    num_frames: u32,
    exposure_time: Duration
}

// impl StatefulCapture for SequenceCapture {
//     type Data = u32;

//     async fn run(&self, detector_handle: DetectorCaptureHandle) -> Result<impl Stream<Item = SLImage>, SLError> {
//         //self.setup(detector_handle.clone(), self.acquisition_settings).await?;
//         let num_frames = self.num_frames;
//         detector_handle.set_exposure_mode(ExposureModes::SequenceMode).await?;
//         detector_handle.set_exposure_time(self.exposure_time).await?;
//         detector_handle.set_number_of_frames(self.num_frames).await?;
//         let (x, y) = detector_handle.get_image_dims().await?;
//         let timeout = self.acquisition_settings.timeout;

//         let capture = self.clone();

//        Ok(stream! {
//             let mut frame = 0;
//             let data = Arc::new(tokio::sync::Mutex::new(vec![0u16; (x * y) as usize]));
//             while frame < num_frames {
//                 match detector_handle.acquire_image(Arc::clone(&data), Some(timeout)).await {
//                     Ok(buffer_info) => {
//                         yield SLImage::new(100, 100);
//                         // report_watch_tx.send(CaptureReport {
//                         //     buffer_info,
//                         //     frame,
//                         //     capture: CaptureMode::Sequence(capture.clone())
//                         // });
//                         frame += 1;
//                     },
//                     Err(e) => {},
//                 }
//             }
//             detector_handle.stop_stream().await;
//         })

//         // let final_stream = stream! {
//         //     pin!(stream);
//         //     loop {
//         //         tokio::select! {
//         //             Some(cmd_message) = rx.recv() => {
//         //                 match cmd_message {
//         //                     CaptureCommand::Cancel => break,
//         //                 }
//         //             },
//         //             Some(img) = stream.next() => {
//         //                 yield img;
//         //             },
//         //             else => break,
//         //         }}
//         //     };

//         // Ok(final_stream)
//     }
// }

