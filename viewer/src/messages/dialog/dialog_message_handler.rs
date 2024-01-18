use crate::messages::prelude::*;


#[derive(Debug, Default, Clone)]
pub struct DialogMessageHandler {
    select_detector_dialog: SelectDetectorMessageHandler,
}

impl MessageHandler<DialogMessage, ()> for DialogMessageHandler {
    fn process_message(&mut self, message: DialogMessage, responses: &mut VecDeque<Message>, data: ()) {
        match message {
            DialogMessage::SelectDetectorDialog(message) => self.select_detector_dialog.process_message(message, responses, data)
        }
    }
}