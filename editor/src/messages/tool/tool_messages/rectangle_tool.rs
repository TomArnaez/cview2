#[derive(Default)]
pub struct RectangleTool {
    fsm_state: RectangleToolFsmState,
    tool_data: RectangleToolData,
    options: RectangleToolOptions
}

pub struct RectangleToolOptions {
    line_weight: f64,
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