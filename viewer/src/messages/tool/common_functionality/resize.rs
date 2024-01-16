use crate::messages::input_mapper::utility_types::input_mouse::ViewportPosition;

#[derive(Clone, Debug, Default)]
pub struct Resize {
	drag_start: ViewportPosition,
}

impl Resize {
}