
use crate::messages::prelude::*;
use super::tool_messages::*;
use crate::messages::layout::utility_types::widget_prelude::*;

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

    	/// When an event makes the tool change or do something, it is processed here to perform a step (transition) on the tool's finite state machine (FSM).
	/// This function is called by the specific tool's message handler when the dispatcher routes a message to the active tool.
	fn process_event(
		&mut self,
		message: ToolMessage,
		tool_data: &mut Self::ToolData,
		transition_data: &mut ToolActionHandlerData,
		options: &Self::ToolOptions,
		responses: &mut VecDeque<Message>,
		update_cursor_on_transition: bool,
	) where
		Self: PartialEq + Sized + Copy,
	{
		// Transition the tool
		let new_state = self.transition(message, tool_data, transition_data, options, responses);

		// Update state
		if *self != new_state {
			*self = new_state;
		}
	}
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

impl ToolData {
    pub fn active_tool_mut(&mut self) -> &mut Box<Tool> {
        self.tools.get_mut(&self.active_tool_type).expect("The active tool is not initialized")
    }

    pub fn active_tool(&self) -> &Tool {
		self.tools.get(&self.active_tool_type).map(|x| x.as_ref()).expect("The active tool is not initialized")
	}
}

impl LayoutHolder for ToolData {
    fn layout(&self) -> Layout {
        let tool_layout = list_tools().iter().map(|tool| {
            IconButton::new(tool.icon_name(), 32)
            .disabled(false)
            .tooltip(tool.tooltip().clone())
            .widget_holder()
        }).collect();

        Layout::WidgetLayout(WidgetLayout {
            layout: vec![LayoutGroup::Row { widgets: tool_layout}]
        })
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
                tools: list_tools().into_iter().map(|tool| {
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

fn list_tools() -> Vec<Box<Tool>> {
    vec![
        Box::<select_tool::SelectTool>::default(),
        Box::<rectangle_tool::RectangleTool>::default(),
    ]
}