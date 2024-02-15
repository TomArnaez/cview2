use std::time::Duration;

use specta::Type;
use tauri_specta::Event;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wrapper::{FullWellModes, ROI};
use chrono::{DateTime, Utc};

#[tauri::command]
#[specta::specta]
pub fn run_capture(capture: Capture) {

}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
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

#[derive(Debug, Clone, Serialize, Type, Event)]
pub struct CaptureProgressEvent {
    pub id: Uuid,
    pub task_count: u32,
    pub completed_task_count: u32,
    pub phase: String,
    pub message: String,
    pub estimated_completation: DateTime<Utc>
}
