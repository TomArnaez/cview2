use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wrapper::{FullWellModes, ROI};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capture {
    Stream,
    Sequence,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct CaptureSettings {
    dds_on: bool,
    full_well_mode: FullWellModes,
    roi: ROI,
    test_mode: bool,
    timeout: Duration,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaptureProgressEvent {
    pub id: Uuid,
    pub task_count: u32,
    pub completed_task_count: u32,
    pub phase: String,
    pub message: String,
    pub estimated_completation: DateTime<Utc>
}
