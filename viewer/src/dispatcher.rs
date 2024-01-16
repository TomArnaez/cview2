use crate::messages::prelude::*;

pub struct Dispatcher {
    message_queues: Vec<VecDeque<Message>>,
    pub responses: Vec<FrontendMessage>,
    pub message_handlers: DispatcherMessageHandlers,
}

impl Default for Dispatcher {
    fn default() -> Self {
        let mut responses = Vec::new();
        let detector_message_handler = DetectorMessageHandler::new(&mut responses);


        let message_handlers = DispatcherMessageHandlers {
            detector_message_handler,
            tool_message_handler: ToolMessageHandler::default()
        };

        Self {
            responses,
            message_handlers,
            message_queues: Vec::new()
        }
    }
}

pub struct DispatcherMessageHandlers  {
    detector_message_handler: DetectorMessageHandler,
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
                Message::NoOp => {},
                Message::Init => {},
                Message::Detector(message) => {
                    self.message_handlers.detector_message_handler.process_message(message, &mut queue, ())
                },
                Message::Debug(message) => {
                }
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