use serde::{Serialize, Deserialize};

use crate::messages::tool::utility_types::{ToolFrontendState, ToolType};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum FrontendMessage {
    DisplayDialog {
        title: String
    },
    SetActiveTool(ToolType),
    TriggerViewportResize,
}