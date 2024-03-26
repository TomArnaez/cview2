use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug)]
pub enum CaptureReportUpdate {
    TaskCount(usize),
    CompletedTaskCount(usize),
    Message(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct CaptureReport {
    pub id: Uuid,
    pub name: String,

    pub status: CaptureStatus,
    pub task_count: usize,
    pub completed_task_count: usize,

    pub message: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
pub enum CaptureStatus {
    Initialised,
    Running,
    Completed,
    Canceled,
    Failed,
    CompletedWithErrors,
}

pub struct CaptureReportBuilder {
    pub id: Uuid,
    pub name: String,
}

impl CaptureReportBuilder {
    pub fn build(&self) -> CaptureReport {
        CaptureReport {
            id: self.id,
            name: self.name.clone(),
            task_count: 0,
            completed_task_count: 0,
            status: CaptureStatus::Initialised,
            message: String::new()
        }
    }

    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
