use crate::messages::layout::utility_types::widget_prelude::*;
use crate::messages::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct LayoutMessageHandler {
	layouts: [Layout; LayoutTarget::LayoutTargetLength as usize],
}

impl MessageHandler<LayoutMessage, ()> for LayoutMessageHandler {
	fn process_message(&mut self, message: LayoutMessage, responses: &mut VecDeque<Message>, data: ()) {
		
	}
}