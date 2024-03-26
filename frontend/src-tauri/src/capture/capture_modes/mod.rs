mod defect_map;
mod helpers;
mod sequence;
mod signal_accumulation;
mod stream;

use std::sync::Arc;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;
use tokio::sync::Mutex;

pub use self::sequence::SequenceCapture;
use super::{detector::{CorrectionImages, DetectorCaptureHandle}, report::CaptureReportUpdate, capture::StatefulCapture};

#[enum_dispatch(StatefulCapture)]
#[derive(Debug, Deserialize, Serialize, Type)]
pub enum CaptureMode {
    SequenceCapture,
}

// All the outside references the capture needs
pub struct CaptureContext {
    pub app: AppHandle,
    pub correction_images: Arc<Mutex<CorrectionImages>>,
    pub events_tx: async_channel::Sender<CaptureReportUpdate>,
    pub detector_handle: DetectorCaptureHandle,
}