use crate::messages::prelude::*;
use crate::messages::layout::utility_types::widget_prelude::*;

#[derive(Debug, Clone, Default)]
pub struct MenuBarMessageHandler {
}

impl MessageHandler<MenuBarMessage, ()> for MenuBarMessageHandler {
    fn process_message(&mut self, message: MenuBarMessage, responses: &mut VecDeque<Message>, data: ()) {

        match message {
            SendLayout => self.send_layout(responses, LayoutTarget::MenuBar),
        }
    }
}