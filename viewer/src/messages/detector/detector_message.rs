use capture::CaptureMode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum DetectorMessage {
    StartCapture(CaptureMode),
    StopCapture,
    SendSoftwareTrigger
}