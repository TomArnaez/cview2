use crate::messages::prelude::*;

use super::utility_types::ToolFsmState;

#[derive(Debug, Default)]
pub struct ToolMessageHandler {
    pub tool_state: ToolFsmState,
}

impl MessageHandler<ToolMessage, ()> for ToolMessageHandler {
    fn process_message(&mut self, message: ToolMessage, responses: &mut VecDeque<Message>, data: ()) {
        match message {
            ToolMessage::ActivateTool { tool_type } => {
                let tool_data = &mut self.tool_state.tool_data;
                let old_tool = tool_data.active_tool_type;

                // Do nothing if switching to the same tool
                if tool_type == old_tool {
                    return;
                }

                tool_data.active_tool_type = tool_type;
            },
            _  => {}
        }
    }
}