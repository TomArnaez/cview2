use serde::{Deserialize, Serialize};

use crate::messages::prelude::*;

#[derive(Debug)]
pub enum Message {
    Tool(ToolMessage)
}