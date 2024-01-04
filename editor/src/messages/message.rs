use serde::{Deserialize, Serialize};

use crate::messages::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Debug(DebugMessage),
    Tool(ToolMessage)
}