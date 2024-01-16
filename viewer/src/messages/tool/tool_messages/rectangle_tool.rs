use super::tool_prelude::*;

use crate::messages::tool::common_functionality::resize::Resize;


#[derive(Default)]
pub struct RectangleTool {
    fsm_state: RectangleToolFsmState,
    tool_data: RectangleToolData,
    options: RectangleToolOptions
}

pub struct RectangleToolOptions {
    line_weight: f64,
    stroke: Colour
}

impl Default for RectangleToolOptions {
    fn default() -> Self {
        Self {
            line_weight: 5.,
			stroke: Colour { hue: 0., saturation: 0., luminance: 0.}
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum RectangleOptionsUpdate {
    LineWeight(f64),
    StrokeColour(Option<Colour>),
    WorkingColour(Option<Colour>, Option<Colour>)
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum RectangleToolMessage {
    Overlays,
	Abort,
    WorkingColourChanged,

    // Tool-specific messages
    DragStart,
    DragStop,
    UpdateOptions(RectangleOptionsUpdate)
}

#[derive(Clone, Copy, Debug, Default)]
enum RectangleToolFsmState {
	#[default]
	Ready,
	Drawing,
	WorkingColourChanged
}

#[derive(Clone, Debug, Default)]
struct RectangleToolData {
	data: Resize,
}

impl Fsm for RectangleToolFsmState {
	type ToolData = RectangleToolData;
	type ToolOptions = RectangleToolOptions;

	fn transition(self, event: ToolMessage, tool_data: &mut Self::ToolData, ToolActionHandlerData { image, .. }: &mut ToolActionHandlerData, options: &Self::ToolOptions, responses: &mut VecDeque<Message>) -> Self {
		let shape_data = &mut tool_data.data;

		let ToolMessage::Rectangle(event) = event else {
			return self;
		};

		match (self, event) {
			(_, RectangleToolMessage::Overlays) => {
					self
			},
			(RectangleToolFsmState::Ready, RectangleToolMessage::DragStart) => {
				RectangleToolFsmState::Drawing
			},
			(RectangleToolFsmState::Drawing, RectangleToolMessage::DragStop) => {
				RectangleToolFsmState::Ready
			},
			(RectangleToolFsmState::Drawing, RectangleToolMessage::Abort) => {
				RectangleToolFsmState::Ready
			},
			(_, RectangleToolMessage::WorkingColourChanged) => {
				self
			},
			_ => self,
		}
	}
}