use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::capture::capture::CaptureSettings;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamCapture {
    capture_settings: CaptureSettings,
    stream_time: Option<Duration>,
}