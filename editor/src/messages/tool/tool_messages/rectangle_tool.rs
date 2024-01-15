use crate::{utility_traits::MessageHandler, messages::tool::utility_types::ToolActionHandlerData};

use super::tool_prelude::ToolMessage;

#[derive(Default)]
pub struct RectangleTool {
    fsm_state: RectangleToolFsmState,
    tool_data: RectangleToolData,
    options: RectangleToolOptions
}

pub struct RectangleToolOptions {
    line_weight: f64,
    stroke: ToolColorOptions
}

impl Default for RectangleToolOptions {
    fn default() -> Self {
        Self {
            line_weight: 5.,
        }
    }
}

pub enum RectangleOptionsUpdate {
    LineWeight(f64),
    StrokeColour(Option<Colour>),
    StrokeColourType(ToolColourType),
    WorkingColour(Option<Colour>, Option<Colour>)
}

pub enum RectangleToolMessage {
    Overlays,
    WorkingColourChanged,

    // Tool-specific messages
    DragStart,
    DragStop,
    UpdateOptions(RectangleOptionsUpdate)
}

impl<'a> MessageHandler<ToolMessage, &mut ToolActionHandlerData<'a>> for RectangleTool {
	fn process_message(&mut self, message: ToolMessage, responses: &mut VecDeque<Message>, tool_data: &mut ToolActionHandlerData<'a>) {
		let ToolMessage::Rectangle(RectangleToolMessage::UpdateOptions(action)) = message else {
			self.fsm_state.process_event(message, &mut self.tool_data, tool_data, &self.options, responses, true);
			return;
		};

		match action {
			RectangleOptionsUpdate::FillColor(color) => {
				self.options.fill.custom_color = color;
				self.options.fill.color_type = ToolColorType::Custom;
			}
			RectangleOptionsUpdate::FillColorType(color_type) => self.options.fill.color_type = color_type,
			RectangleOptionsUpdate::LineWeight(line_weight) => self.options.line_weight = line_weight,
			RectangleOptionsUpdate::StrokeColor(color) => {
				self.options.stroke.custom_color = color;
				self.options.stroke.color_type = ToolColorType::Custom;
			}
			RectangleOptionsUpdate::StrokeColorType(color_type) => self.options.stroke.color_type = color_type,
			RectangleOptionsUpdate::WorkingColors(primary, secondary) => {
				self.options.stroke.primary_working_color = primary;
				self.options.stroke.secondary_working_color = secondary;
				self.options.fill.primary_working_color = primary;
				self.options.fill.secondary_working_color = secondary;
			}
		}

		self.send_layout(responses, LayoutTarget::ToolOptions);
	}

	fn actions(&self) -> ActionList {
		use RectangleToolFsmState::*;

		match self.fsm_state {
			Ready => actions!(RectangleToolMessageDiscriminant;
				DragStart,
				PointerMove,
			),
			Drawing => actions!(RectangleToolMessageDiscriminant;
				DragStop,
				Abort,
				PointerMove,
			),
		}
	}
}