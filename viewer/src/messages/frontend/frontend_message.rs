use serde::{Serialize, Deserialize};

use crate::messages::tool::utility_types::{ToolType};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum FrontendMessage {
    DisplayDialog {
        title: String
    },
    SetActiveTool(ToolType),
    TriggerViewportResize,
}