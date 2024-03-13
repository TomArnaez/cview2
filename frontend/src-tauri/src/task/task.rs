use std::{
    fmt,
    future::{Future, IntoFuture},
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc,
    },
    task::{Context, Poll},
};

use async_channel as chan;
use async_trait::async_trait;
use chan::{Recv, RecvError};
use downcast_rs::{impl_downcast, Downcast};
use log::{trace, warn};
use tokio::sync::oneshot;
use uuid::Uuid;

use super::error::{RunError, SystemError};

pub type TaskId = Uuid;

pub trait AnyTaskOutput: Send + fmt::Debug + Downcast + 'static {}

impl_downcast!(AnyTaskOutput);

impl<T: fmt::Debug + Send + 'static> AnyTaskOutput for T {}

pub trait IntoAnyTaskOutput {
    fn into_output(self) -> TaskOutput;
}

#[derive(Debug)]
pub enum TaskOutput {
    Out(Box<dyn AnyTaskOutput>),
    Empty,
}

#[derive(Debug)]
pub enum TaskStatus<E: RunError> {
    /// The task has finished successfully and maybe has some output for the user.
    Done(TaskOutput),
    /// Task was gracefully cancelled by the user.
    Canceled,
    /// Task was forcefully aborted by the user.
    ForcedAbortion,
    Shutdown(Box<dyn Task<E>>),
    /// Task had and error so we return it back and the user can handle it appropriately.
    Error(E),
}

#[derive(Debug)]
pub enum ExecStatus {
    Done(TaskOutput),
    Canceled,
}

#[derive(Debug)]
pub enum InternalTaskExecStatus<E: RunError> {
    Done(TaskOutput),
    Canceled,
    Error(E),
}

impl<E: RunError> From<Result<ExecStatus, E>> for InternalTaskExecStatus<E> {
    fn from(result: Result<ExecStatus, E>) -> Self {
        result.map_or_else(Self::Error, |status| match status {
            ExecStatus::Done(out) => Self::Done(out),
            ExecStatus::Canceled => Self::Canceled,
        })
    }
}

/// A helper trait to convert any type that implements [`Task<E>`] into a [`Box<dyn Task<E>>`], boxing it.
pub trait IntoTask<E>: Send {
    fn into_task(self) -> Box<dyn Task<E>>;
}

/// Blanket implementation for all types that implements [`Task<E>`] and `'static`
impl<T: Task<E> + 'static, E: RunError> IntoTask<E> for T {
    fn into_task(self) -> Box<dyn Task<E>> {
        Box::new(self)
    }
}

#[async_trait]
pub trait Task<E: RunError>: fmt::Debug + Downcast + Send + 'static {
    async fn run(&mut self, interrupter: &Interrupter) -> Result<ExecStatus, E>;
    fn id(&self) -> TaskId;
}

impl_downcast!(Task<E> where E: RunError);

#[pin_project::pin_project]
pub struct InterrupterFuture<'recv> {
    #[pin]
    fut: Recv<'recv, InterruptionRequest>,
    has_interrupted: &'recv AtomicU8,
}

impl Future for InterrupterFuture<'_> {
    type Output = InterruptionKind;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx) {
            Poll::Ready(Ok(InterruptionRequest { kind, ack })) => {
                if ack.send(Ok(())).is_err() {
                    warn!("TaskInterrupter ack channel closed");
                }
                this.has_interrupted.store(kind as u8, Ordering::Relaxed);
                Poll::Ready(kind)
            }
            Poll::Ready(Err(RecvError)) => {
                // In case the task handle was dropped, we can't receive any more interrupt messages
                // so we will never interrupt and the task will run freely until ended
                warn!("Task interrupter channel closed, will run task until it finishes!");
                Poll::Pending
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'recv> IntoFuture for &'recv Interrupter {
    type Output = InterruptionKind;

    type IntoFuture = InterrupterFuture<'recv>;

    fn into_future(self) -> Self::IntoFuture {
        InterrupterFuture {
            fut: self.interrupt_rx.recv(),
            has_interrupted: &self.has_interrupted,
        }
    }
}

#[derive(Debug)]
pub struct Interrupter {
    interrupt_rx: chan::Receiver<InterruptionRequest>,
    has_interrupted: AtomicU8,
}

impl Interrupter {
    pub(crate) fn new(interrupt_tx: chan::Receiver<InterruptionRequest>) -> Self {
        Self {
            interrupt_rx: interrupt_tx,
            has_interrupted: AtomicU8::new(0),
        }
    }

    /// Check if the user requested a pause or a cancel, returning the kind of interruption that was requested
    /// in a non-blocking manner.
    pub fn try_check_interrupt(&self) -> Option<InterruptionKind> {
        InterruptionKind::load(&self.has_interrupted).map_or_else(
            || {
                if let Ok(InterruptionRequest { kind, ack }) = self.interrupt_rx.try_recv() {
                    if ack.send(Ok(())).is_err() {
                        warn!("TaskInterrupter ack channel closed");
                    }

                    self.has_interrupted.store(kind as u8, Ordering::Relaxed);

                    Some(kind)
                } else {
                    None
                }
            },
            Some,
        )
    }

    pub(super) fn reset(&self) {
        self.has_interrupted
            .compare_exchange(
                InterruptionKind::Cancel as u8,
                0,
                Ordering::Release,
                Ordering::Relaxed,
            )
            .expect("we must only reset paused tasks");
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptionKind {
    Cancel = 1,
}

impl InterruptionKind {
    fn load(kind: &AtomicU8) -> Option<Self> {
        match kind.load(Ordering::Relaxed) {
            1 => Some(Self::Cancel),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct InterruptionRequest {
    kind: InterruptionKind,
    ack: oneshot::Sender<Result<(), SystemError>>,
}

#[derive(Debug)]
pub struct TaskHandle<E: RunError> {
    pub(crate) worktable: Arc<TaskWorktable>,
    pub(crate) done_rx: oneshot::Receiver<Result<TaskStatus<E>, SystemError>>,
    pub(crate) task_id: TaskId,
}

impl<E: RunError> Future for TaskHandle<E> {
    type Output = Result<TaskStatus<E>, SystemError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.done_rx)
            .poll(cx)
            .map(|res| res.expect("TaskHandle done channel unexpectedly closed"))
    }
}

impl<E: RunError> TaskHandle<E> {
    /// Get the unique identifier of the task
    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub async fn cancel(&self) -> Result<(), SystemError> {
        let is_canceled = self.worktable.is_canceled.load(Ordering::Relaxed);
        let is_done = self.worktable.is_done.load(Ordering::Relaxed);

        trace!("Received cancel command task: <is_canceled={is_canceled}, is_done={is_done}>");

        if !is_canceled && !is_done {
            if self.worktable.is_running.load(Ordering::Relaxed) {
                let (tx, rx) = oneshot::channel();

                trace!("Task is running, sending cancel request");

                self.worktable.cancel(tx).await;

                rx.await.expect("Worker failed to ack cancel request")?;
            } else {
                trace!("Task is not running, setting is_canceled flag");
                self.worktable.is_canceled.store(true, Ordering::Relaxed);
            }
        }

        Ok(())
    }

    pub async fn force_abortion(&self) -> Result<(), SystemError> {
        self.worktable.set_aborted();
        Ok(())
    }

    pub async fn resume(&self) -> Result<(), SystemError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TaskWorktable {
    started: AtomicBool,
    is_running: AtomicBool,
    is_done: AtomicBool,
    is_paused: AtomicBool,
    is_canceled: AtomicBool,
    is_aborted: AtomicBool,
    interrupt_tx: chan::Sender<InterruptionRequest>,
}

impl TaskWorktable {
    pub fn new(interrupt_tx: chan::Sender<InterruptionRequest>) -> Self {
        Self {
            started: AtomicBool::new(false),
            is_running: AtomicBool::new(false),
            is_done: AtomicBool::new(false),
            is_paused: AtomicBool::new(false),
            is_canceled: AtomicBool::new(false),
            is_aborted: AtomicBool::new(false),
            interrupt_tx,
        }
    }

    pub fn set_started(&self) {
        self.started.store(true, Ordering::Relaxed);
        self.is_running.store(true, Ordering::Relaxed);
    }

    pub fn set_completed(&self) {
        self.is_done.store(true, Ordering::Relaxed);
        self.is_running.store(false, Ordering::Relaxed);
    }

    pub fn set_unpause(&self) {
        self.is_paused.store(false, Ordering::Relaxed);
    }

    pub fn set_aborted(&self) {
        self.is_aborted.store(true, Ordering::Relaxed);
    }

    pub async fn cancel(&self, tx: oneshot::Sender<Result<(), SystemError>>) {
        self.is_canceled.store(true, Ordering::Relaxed);
        self.is_running.store(false, Ordering::Relaxed);

        self.interrupt_tx
            .send(InterruptionRequest {
                kind: InterruptionKind::Cancel,
                ack: tx,
            })
            .await
            .expect("Worker channel closed trying to pause task");
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::Relaxed)
    }

    pub fn is_canceled(&self) -> bool {
        self.is_canceled.load(Ordering::Relaxed)
    }

    pub fn is_aborted(&self) -> bool {
        self.is_aborted.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
pub struct TaskWorkState<E: RunError> {
    pub(crate) task: Box<dyn Task<E>>,
    pub(crate) worktable: Arc<TaskWorktable>,
    pub(crate) done_tx: oneshot::Sender<Result<TaskStatus<E>, SystemError>>,
    pub(crate) interrupter: Arc<Interrupter>,
}
