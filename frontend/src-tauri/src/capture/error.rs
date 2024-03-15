use thiserror::Error;
use wrapper::SLError;

use super::DetectorId;

#[derive(Error, Debug)]
pub enum DetectorControllerError {
    #[error("Capture in progress")]
    CaptureInProgress,
    #[error("Detector disconnected")]
    DetectorDisconnected,
    #[error("Detector not found for id: {0}")]
    DetectorNotFound(DetectorId),
    #[error("No capture in progress")]
    NoCaptureInProgress,
}

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("Capture cancelled")]
    Canceled,
    #[error("critical critical error: {0}")]
    Critical(&'static str),
    #[error("Internal SDK Error")]
    SLError(#[from] SLError),
}
