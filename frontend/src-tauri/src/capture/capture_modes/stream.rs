use std::time::Duration;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::capture::capture::CaptureSettings;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct StreamCapture {
    capture_settings: CaptureSettings,
    stream_time: Option<Duration>,
}