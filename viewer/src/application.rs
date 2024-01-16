use crate::{dispatcher::Dispatcher, messages::{prelude::Message, frontend::FrontendMessage}};
use std::sync::mpsc::{Sender, Receiver, self};

pub struct Viewer {
    pub dispatcher: Dispatcher,
    tx: Sender<FrontendMessage>,
}

impl Viewer {
    pub fn new() -> (Viewer, Receiver<FrontendMessage>) {
        let (tx, rx) = mpsc::channel();
        return ( Self { dispatcher: Dispatcher::new(), tx }, rx );
    }

    pub fn handle_message<T: Into<Message>>(&mut self, message: T) {
        self.dispatcher.handle_message(message);

        while let Some(response) = self.dispatcher.responses.pop() {
            self.tx.send(response).unwrap();
        }
    }
}