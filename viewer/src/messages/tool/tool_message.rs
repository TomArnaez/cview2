use serde::{Deserialize, Serialize};

use crate::messages::prelude::*;

use super::utility_types::ToolType;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, specta::Type)]
pub enum ToolMessage {
    
    // SubMessages
    Select(SelectToolMessage),
    Line(LineToolMessage),
    Rectangle(RectangleToolMessage),
    Ellipse,

    // Messages
    ActivateTool {
        tool_type: ToolType,
    },
    
    InitTools,
}