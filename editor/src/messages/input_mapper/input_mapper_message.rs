use crate::messages::input_mapper::utility_types::input_mouse::MouseButton;


pub enum InputMapperMessage {
    DoubleClick(MouseButton),

    PointerMove,
    WheelScroll,
}