use serde::{Deserialize, Serialize};

use crate::messages::prelude::*;

#[derive(Debug)]
pub enum Message {
    Debug(DebugMessage),
    Tool(ToolMessage)
}