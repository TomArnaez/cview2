use crate::messages::prelude::*;

use super::utility_types::ToolType;

#[derive(PartialEq, Debug)]
pub enum ToolMessage {
    
    // SubMessages
    Select(SelectToolMessage),

    // Messages
    ActivateTool {
        tool_type: ToolType,
    },
}