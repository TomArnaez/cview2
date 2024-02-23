use std::{time::Duration, sync::{Arc, Mutex}};
use specta::Type;
use tauri_specta::Event;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use tokio::sync::mpsc;
use uuid::Uuid;
use wrapper::{ExposureModes, FullWellModes, SLBufferInfo, SLError, ROI};
use chrono::{DateTime, Utc};

use super::detector::DetectorCaptureHandle;

#[async_trait]
pub trait Capture: {
    async fn setup(&self, detector_handle: DetectorCaptureHandle, acquisition_settings: CaptureSettings) -> Result<(), SLError> {
        detector_handle.set_dds(acquisition_settings.dds_on).await?;
        detector_handle.set_full_well_mode(acquisition_settings.full_well_mode).await?;
        detector_handle.set_roi(acquisition_settings.roi).await?;
        detector_handle.set_test_mode(acquisition_settings.test_mode).await?;
        
        Ok(())
    }

    async fn run(&self, detector_handle: DetectorCaptureHandle, rx: mpsc::Receiver<CaptureCommand>) -> Result<mpsc::Receiver<CaptureResponse>, SLError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum CaptureMode {
    Stream(StreamCapture),
    Sequence(SequenceCapture),
}

#[derive(Debug)]
pub enum CaptureCommand {
    Cancel
}

#[derive(Debug)]
pub enum CaptureResponse {
    Error(SLError),
    Image(SLBufferInfo),
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

// #[async_trait]
// impl Capture for SequenceCapture {
//     async fn run(&self, detector_handle: DetectorCaptureHandle, mut rx: mpsc::Receiver<CaptureCommand>) -> Result<mpsc::Receiver<CaptureResponse>, SLError> {
//         self.setup(detector_handle.clone(), self.acquisition_settings).await?;
//         let num_frames = self.num_frames;
//         detector_handle.set_exposure_mode(ExposureModes::SequenceMode).await?;
//         detector_handle.set_exposure_time(self.exposure_time).await?;
//         detector_handle.set_number_of_frames(self.num_frames).await?;
//         let (x, y) = detector_handle.get_image_dims().await?;
//         //let images = SLImage::new_stack(x, y, self.num_frames);
//         let timeout = self.acquisition_settings.timeout;

//         let (acq_tx, acq_rx) = mpsc::channel(10);
//         tokio::spawn(async move {
//             let mut count = 0;
//             let data = Arc::new(Mutex::new(vec![0u16; (x * y) as usize]));
//             while count < num_frames {
//                 match detector_handle.acquire_image(Arc::clone(&data), Some(timeout)).await {
//                     Ok(buffer_info) => {
//                         acq_tx.send(CaptureResponse::Image(buffer_info)).await.unwrap();
//                         count += 1;
//                     },
//                     Err(e) => acq_tx.send(CaptureResponse::Error(e)).await.unwrap()
//                 }
//             }
//             detector_handle.stop_stream().await;
//         });

//         Ok(acq_rx)
//     }
// }

#[derive(Debug, Clone, Serialize, Type, Event)]
pub struct CaptureProgressEvent {
    pub id: Uuid,
    pub task_count: u32,
    pub completed_task_count: u32,
    pub phase: String,
    pub message: String,
}
