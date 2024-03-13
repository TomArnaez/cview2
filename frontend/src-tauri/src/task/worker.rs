use std::{cell::RefCell, sync::Arc};
use tokio::{sync::oneshot, task::JoinHandle};
use super::{error::RunError, task::{Task, TaskHandle, TaskWorktable}};

#[derive(Debug)]
pub struct Worker {
}

impl Worker {
    pub async fn add_task<E: RunError>(&self, new_task: Box<dyn Task<E>>) -> TaskHandle<E> {
        let (done_tx, done_rx) = oneshot::channel();

        let (interrupt_tx, interrupt_rx) = async_channel::bounded(1);

        let worktable = Arc::new(TaskWorktable::new(interrupt_tx));

        let task_id = new_task.id();

        TaskHandle {
            worktable,
            done_rx,
            task_id,
        }
    }
}