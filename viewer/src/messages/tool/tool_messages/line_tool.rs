use super::tool_prelude::*;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct LineTool {
    fsm_state: LineToolFsmState,
    tool_data: LineToolData,
    options: LineOptions
}

pub struct LineOptions {
}

impl Default for LineOptions {
    fn default() -> Self {
        Self {
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum LineToolMessage {
    Abort,
    WorkingColourChanged,

    DragStart,
    DragStop,
    UpdateOptions(LineOptionsUpdate),
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum LineOptionsUpdate {
}

impl ToolMetadata for LineTool {
    fn icon_name(&self) -> String {
        "VectorLineTool".into()
    }
	fn tooltip(&self) -> String {
		"Line Tool".into()
	}
	fn tool_type(&self) -> crate::messages::tool::utility_types::ToolType {
		ToolType::Line
	}
}

impl<'a> MessageHandler<ToolMessage, &mut ToolActionHandlerData<'a>> for LineTool {
	fn process_message(&mut self, message: ToolMessage, responses: &mut VecDeque<Message>, tool_data: &mut ToolActionHandlerData<'a>) {
		let ToolMessage::Line(LineToolMessage::UpdateOptions(action)) = message else {
			self.fsm_state.process_event(message, &mut self.tool_data, tool_data, &self.options, responses, true);
			return;
		};	}

}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum LineToolFsmState {
	#[default]
	Ready,
	Drawing,
}

#[derive(Clone, Debug, Default)]
struct LineToolData {
	drag_start: Vector2<f32>,
	drag_current: Vector2<f32>,
	angle: f64,
	weight: f64,
}

impl Fsm for LineToolFsmState {
    type ToolData = LineToolData;
	type ToolOptions = LineOptions;

    fn transition(self, event: ToolMessage, tool_data: &mut Self::ToolData, transition_data: &mut ToolActionHandlerData, options: &Self::ToolOptions, responses: &mut VecDeque<Message>) -> Self {
        let ToolMessage::Line(event) = event else {
            return self;
        };
        match (self, event) {
            (_, LineToolMessage::WorkingColourChanged) => {
                self
            }
            _ => self
        }
    }
}