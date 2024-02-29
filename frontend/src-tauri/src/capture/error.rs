use thiserror::Error;
use wrapper::SLError;

#[derive(Error, Debug)]
pub enum DetectorControllerError {
    #[error("Capture in progress")]
    CaptureInProgress,
    #[error("Detector disconnected")]
    DetectorDisconnected,
}

#[derive(Error, Debug)]
pub enum JobError {
    #[error("Capture cancelled")]
    Cancelled,
    #[error("critical job error: {0}")]
	Critical(&'static str),
    #[error("Internal SDK Error")]
    SLError(#[from] SLError)
}