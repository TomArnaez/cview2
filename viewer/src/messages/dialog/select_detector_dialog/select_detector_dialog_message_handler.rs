use crate::messages::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SelectDetectorMessageHandler {

}

impl MessageHandler<SelectDetectorDialogMessage, ()> for SelectDetectorMessageHandler {
    fn process_message(&mut self, message: SelectDetectorDialogMessage, responses: &mut VecDeque<Message>, data: ()) {
    }
}