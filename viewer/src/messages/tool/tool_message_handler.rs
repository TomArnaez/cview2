use crate::messages::prelude::*;

use super::utility_types::{ToolFsmState, ToolActionHandlerData};

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

                // let mut send_abort_to_tool = |tool_type| {
                //     if let Some(tool) = tool_data.tools.get_mut(&tool_type) {
                //         let mut data = ToolActionHandlerData {
                //             image,
                //         };
                //         if let Some(tool_abort_message) = tool.event_to_message_map().tool_abort {
                //             tool.process_message(tool_abort_message, responses, &mut data);
                //         }
                //     };
                // };

                // send_abort_to_tool(tool_type);
                // send_abort_to_tool(tool_type);

                // Unsubscribe old tool from the broadcaster
				tool_data.tools.get(&tool_type).unwrap().deactivate(responses);

                // Store the new active tool
				tool_data.active_tool_type = tool_type;
                
                tool_data.tools.get(&tool_type).unwrap().activate(responses);

                responses.add(Message::Frontend(FrontendMessage::SetActiveTool(tool_data.active_tool_type)));
            },
            ToolMessage::InitTools => {
                let tool_data = &mut self.tool_state.tool_data;
                responses.add(Message::Frontend(FrontendMessage::SetActiveTool(tool_data.active_tool_type)));
            }
            _  => {}
        }
    }
}