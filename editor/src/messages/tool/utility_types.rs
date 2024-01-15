
use crate::messages::{prelude::*, tool::tool_messages::select_tool};

use std::{collections::HashMap, fmt};
use serde::Deserialize;
use specta::Type;

pub struct ToolActionHandlerData {
}

impl ToolActionHandlerData {
    pub fn new() -> Self {
        Self {
        }
    }
}

pub trait ToolCommon: for<'a, 'b> MessageHandler<ToolMessage, &'b mut ToolActionHandlerData> + ToolMetadata {}
impl<T> ToolCommon for T where T: for<'a, 'b> MessageHandler<ToolMessage, &'b mut ToolActionHandlerData> + ToolMetadata {}

type Tool = dyn ToolCommon + Send + Sync;

pub trait Fsm {
    type ToolData;
    type ToolOptions;
}

pub trait ToolMetadata {
    fn icon_name(&self) -> String;
    fn tooltip(&self) -> String;
    fn tool_type(&self) -> ToolType;
}

pub struct ToolData {
    pub active_tool_type: ToolType,
    pub tools: HashMap<ToolType, Box<Tool>>
}

impl fmt::Debug for ToolData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ToolData").field("active_tool_type", &self.active_tool_type).field("tool_options", &"[…]").finish()
    }
}

#[derive(Debug)]
pub struct ToolFsmState {
    pub tool_data: ToolData,
}

impl Default for ToolFsmState {
    fn default() -> Self {
        Self {
            tool_data: ToolData {
                active_tool_type: ToolType::Select,
                tools: list_tool_in_groups().into_iter().map(|tool| {
                    (tool.tool_type(), tool)
                }).collect(),
            }
        }
    }
}

impl ToolFsmState {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Default, Type)]
pub enum ToolType {
    #[default]
    Select,

    // Vector tool group
    Line,
    Rectangle,
    Ellipse,
}

fn list_tool_in_groups() -> Vec<Box<Tool>> {
    vec![
        Box::<select_tool::SelectTool>::default(),
        Box::<
    ]
}