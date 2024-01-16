
use crate::messages::{prelude::*, tool::tool_messages::select_tool};

use std::{collections::HashMap, fmt};
use serde::{Serialize, Deserialize};
use specta::Type;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq, specta::Type)]
pub struct Colour {
    pub hue: f32,
    pub saturation: f32,
    pub luminance: f32
}

pub struct ToolActionHandlerData<'a> {
    pub image: &'a ImageMessageHandler,
}

impl<'a> ToolActionHandlerData<'a> {
    pub fn new(image: &'a ImageMessageHandler) -> Self {
        Self {
            image
        }
    }
}

pub trait ToolCommon: for<'a, 'b> MessageHandler<ToolMessage, &'b mut ToolActionHandlerData<'a>> + ToolMetadata {}
impl<T> ToolCommon for T where T: for<'a, 'b> MessageHandler<ToolMessage, &'b mut ToolActionHandlerData<'a>> + ToolMetadata {}

type Tool = dyn ToolCommon + Send + Sync;

pub trait Fsm {
    type ToolData;
    type ToolOptions;

	/// Implementing this mandatory trait function lets a specific tool react accordingly (and potentially change its state or internal variables) upon receiving an event to do something.
	fn transition(self, message: ToolMessage, tool_data: &mut Self::ToolData, transition_data: &mut ToolActionHandlerData, options: &Self::ToolOptions, responses: &mut VecDeque<Message>) -> Self;
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
    ]
}