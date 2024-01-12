use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize)]
pub enum DetectorMessage {
    StartCapture,
    StopCapture,
    SendSoftareTrigger
}