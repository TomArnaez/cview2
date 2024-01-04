use serde::{Deserialize, Serialize};

use super::tool_prelude::*;

pub struct SelectTool {
    fsm_state: SelectToolFsmState,
    tool_data: SelectToolData
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum SelectToolMessage {
    // Standard messages
    Abort,

    // Tool-specific messages
    DragStart,
    DragStop
}

enum SelectToolFsmState {
    Ready,
    Dragging,
    DrawingBox,
}

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