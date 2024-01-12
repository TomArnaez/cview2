use crate::{messages::{prelude::*,}, utility_traits::MessageHandler};
use log::info;

#[derive(Debug, Default)]
pub struct Dispatcher {
    message_queues: Vec<VecDeque<Message>>,
    pub responses: Vec<FrontendMessage>,
    pub message_handlers: DispatcherMessageHandlers,
}

#[derive(Debug, Default)]
pub struct DispatcherMessageHandlers  {
    tool_message_handler: ToolMessageHandler,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_message<T: Into<Message>>(&mut self, message: T) {
        self.message_queues.push(VecDeque::from_iter([message.into()]));

        while let Some(message) = self.message_queues.last_mut().and_then(VecDeque::pop_front) {

            let mut queue = VecDeque::new();

            match message {

                Message::Tool(message) => {
                }
            }

            if !queue.is_empty() {
                self.message_queues.push(queue);
            }
        }
    }

    /*
    pub fn log_message(&self, message: &Message, queries: &[VecDeque<Message>], message_logging_verbosity: MessageLoggingVerbosity) {
        match message_logging_verbosity {
            MessageLoggingVerbosity::Off => {},
            MessageLoggingVerbosity::Names => {
                info!("Logging names");
            },
            MessageLoggingVerbosity::Contents => {
                info!("Logging contents");
            }
        }
    }
    */
}