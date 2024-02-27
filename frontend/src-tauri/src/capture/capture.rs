use std::{sync::Arc, time::Duration};
use log::info;
use specta::Type;
use serde::{Deserialize, Serialize};
use tokio::{pin, sync::{mpsc, watch}};
use uuid::Uuid;
use wrapper::{ExposureModes, FullWellModes, SLBufferInfo, SLError, SLImage, ROI};
use tokio_stream::{Stream, StreamExt};
use async_stream::stream;
use super::detector::DetectorCaptureHandle;

pub struct Capture {
    id: Uuid,
    report: Option<CaptureReport>,
}

pub struct CaptureState<Capture: StatefulCapture> {
    pub steps: Vec<Capture::Step>,
    pub data: Option<Capture::Data>,
    pub step_number: usize,
}

pub trait StatefulCapture {
    type Data;
    type Step;
    type CaptureResult;

    async fn run(&self, detector_handle: DetectorCaptureHandle) -> Result<impl Stream<Item = SLImage>, SLError>;
    async fn execute_step(&self, data: &Self::Data);
    async fn finalise(&self, data: &Self::Data) -> Self::CaptureResult;
}

pub trait DynCapture: {
    async fn setup(&self, detector_handle: DetectorCaptureHandle, acquisition_settings: CaptureSettings) -> Result<(), SLError> {
        detector_handle.set_dds(acquisition_settings.dds_on).await?;
        detector_handle.set_full_well_mode(acquisition_settings.full_well_mode).await?;
        detector_handle.set_roi(acquisition_settings.roi).await?;
        detector_handle.set_test_mode(acquisition_settings.test_mode).await?;
        
        Ok(())
    }

    fn id(&self) -> Uuid;
    fn report(&self) -> &Option<CaptureReport>;
    async fn run(&self, detector_handle: DetectorCaptureHandle, rx: mpsc::Receiver<CaptureCommand>);
}

impl DynCapture for Capture {
    fn id(&self) -> Uuid {
        self.id
    }

    fn report(&self) -> &Option<CaptureReport> {
        &self.report
    }

    async fn run(&self, detector_handle: DetectorCaptureHandle, rx: mpsc::Receiver<CaptureCommand>) {
        let id = self.id();
        info!("Starting capture <id={id}");
    }
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

