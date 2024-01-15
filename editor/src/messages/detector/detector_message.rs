use capture::CaptureMode;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub enum DetectorMessage {
    StartCapture(CaptureMode),
    StopCapture,
    SendSoftwareTrigger
}