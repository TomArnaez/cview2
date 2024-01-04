use crate::messages::prelude::*;

use super::utility_types::ToolFsmState;

#[derive(Debug, Default)]
pub struct ToolMessageHandler {
    pub tool_state: ToolFsmState,
}