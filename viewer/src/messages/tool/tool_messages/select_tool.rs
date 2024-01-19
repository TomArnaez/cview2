use serde::{Deserialize, Serialize};

use crate::messages::tool::utility_types::ToolActionHandlerData;

use super::tool_prelude::*;

#[derive(Default)]
pub struct SelectTool {
    fsm_state: SelectToolFsmState,
    tool_data: SelectToolData
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum SelectToolMessage {
    // Standard messages
    Abort,

    // Tool-specific messages
    DragStart,
    DragStop
}

impl<'a> MessageHandler<ToolMessage, &mut ToolActionHandlerData<'a>> for SelectTool {
    fn process_message(&mut self, message: ToolMessage, responses: &mut VecDeque<Message>, data: &mut ToolActionHandlerData) {
        
    }
}

#[derive(Default, Debug)]
enum SelectToolFsmState {
    #[default]
    Ready,
    Dragging,
    DrawingBox,
}

#[derive(Default)]
struct SelectToolData {
    
}

impl ToolMetadata for SelectTool {
    fn icon_name(&self) -> String {
        "SelectTool".into()
    }
    fn tooltip(&self) -> String {
        "Select Tool".into()
    }
    fn tool_type(&self) -> crate::messages::tool::utility_types::ToolType {
		ToolType::Select
	}
}

impl ToolTransition for SelectTool {
	fn event_to_message_map(&self) -> EventToMessageMap {
		EventToMessageMap {
			tool_abort: Some(ToolMessage::Select(SelectToolMessage::Abort)),
		}
	}
}