use crate::{dispatcher::Dispatcher, messages::{prelude::Message, frontend::FrontendMessage}};

pub struct Editor {
    pub dispatcher: Dispatcher,
}

impl Editor {
    pub fn new() -> Self {
        Self { dispatcher: Dispatcher::new() }
    }

    pub fn handle_message<T: Into<Message>>(&mut self, message: T) -> Vec<FrontendMessage> {
        self.dispatcher.handle_message(message);

        let mut responses = Vec::new();
        std::mem::swap(&mut responses, &mut self.dispatcher.responses);

        responses
    }
}