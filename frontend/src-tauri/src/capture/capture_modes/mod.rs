mod defect_map;
mod helpers;
mod sequence;
mod signal_accumulation;
mod stream;

use std::sync::Arc;
use enum_dispatch::enum_dispatch;
use tauri::AppHandle;
use tokio::sync::Mutex;

use self::{sequence::SequenceCapture, stream::StreamCapture};
use super::{detector::{CorrectionImages, DetectorCaptureHandle}, report::CaptureReportUpdate};

#[enum_dispatch(StatefulCapture)]
pub enum CaptureMode {
    SequenceCapture,
    StreamCapture
}

// All the outside references a capture needs
pub struct CaptureContext {
    pub app: AppHandle,
    pub correction_images: Arc<Mutex<CorrectionImages>>,
    pub events_tx: async_channel::Sender<CaptureReportUpdate>,
    pub detector_handle: DetectorCaptureHandle,
}