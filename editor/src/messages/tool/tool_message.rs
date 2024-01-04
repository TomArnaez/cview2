use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum ToolMessage {

    SelectTool(SelectToolMessage)
}