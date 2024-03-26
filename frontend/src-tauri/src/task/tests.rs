use std::sync::Mutex;

use async_trait::async_trait;
use log::info;
use thiserror::Error;

use crate::{
    capture::DetectorController,
    task::{task::InterruptionRequest, worker::Worker},
};

use super::task::{ExecStatus, Interrupter, InterruptionKind, Task, TaskId};

#[derive(Debug, Error)]
pub enum SampleError {
    #[error("Sample error")]
    SampleError,
}

#[derive(Debug)]
pub struct CaptureTask {
    id: TaskId,
    data: Mutex<u32>,
}

impl Default for CaptureTask {
    fn default() -> Self {
        Self {
            id: TaskId::new_v4(),
            data: Mutex::new(5),
        }
    }
}

#[async_trait]
impl Task<SampleError> for CaptureTask {
    fn id(&self) -> TaskId {
        self.id
    }

    async fn run(&mut self, interrupter: &Interrupter) -> Result<ExecStatus, SampleError> {
        match interrupter.await {
            InterruptionKind::Cancel => {
                info!("Canceling NeverTask <id='{}'>", self.id);
                Ok(ExecStatus::Canceled)
            }
        }
    }
}

#[tokio::test]
async fn cancel_test() {
    let worker = Worker {};
    let mut task = CaptureTask::default();
    let handle = worker.add_task(Box::new(task)).await;
    let (tx, rx) = async_channel::unbounded();
    let interrupter = Interrupter::new(rx);

    loop {}
}
