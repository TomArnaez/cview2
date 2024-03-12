use specta::Type;
use uuid::Uuid;

#[derive(Debug)]
pub enum CaptureReportUpdate {
    TaskCount(usize),
    CompletedTaskCount(usize),
}

#[derive(Debug, Type, Clone)]
pub struct CaptureReport {
    pub id: Uuid,
    pub name: String,

    pub task_count: usize,
    pub completed_task_count: usize
}