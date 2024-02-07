use std::{sync::{mpsc::Receiver, Mutex, Arc}, thread};
use futures_util::StreamExt;

use capture::{DetectorController, DetectorEvent, CaptureHandle};

use crate::messages::prelude::*;

pub struct DetectorMessageHandler {
    detector_controller: DetectorController,
    handle: Arc<Mutex<Option<Box<dyn CaptureHandle + Send>>>>,
}

impl DetectorMessageHandler {
    pub fn new(responses: &mut Vec<FrontendMessage>) -> Self {
        let (detector_controller, rx) = DetectorController::new();

        Self {
            detector_controller,
            handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_handle(&self, new_handle: Box<dyn CaptureHandle + Send>) {
        {
            let mut handle_lock = self.handle.lock().unwrap();
            *handle_lock = Some(new_handle);
        }
        
        let handle_clone = Arc::clone(&self.handle);

        let thread_handle = thread::spawn(move || {
            Self::process_stream(handle_clone);
        });
    }

    fn process_stream(handle: Arc<Mutex<Option<Box<dyn CaptureHandle + Send>>>>) {
        let handle_lock: std::sync::MutexGuard<'_, Option<Box<dyn CaptureHandle + Send>>> = handle.lock().unwrap();
        let mut stream = handle_lock.as_ref().unwrap().get_stream();

        while let Some(item) = stream.next() {

        }
    }

}

impl MessageHandler<DetectorMessage, ()> for DetectorMessageHandler {
    fn process_message(&mut self, message: DetectorMessage, responses: &mut VecDeque<Message>, data: ()) {

    }
}

pub fn detector_thread() {
    loop {
        
    }
}