use crate::messages::input_mapper::utility_types::input_mouse::MouseButton;

use super::utility_types::input_mouse::ViewportBounds;

pub enum InputMapperMessage {
    DoubleClick(MouseButton),
    PointerMove,
    ViewportBounds(Vec<ViewportBounds>),
    WheelScroll,
}