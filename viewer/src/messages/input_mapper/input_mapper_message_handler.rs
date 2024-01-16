use super::{utility_types::input_mouse::{ViewportBounds, MouseState}, input_mapper_message::InputMapperMessage};
use crate::messages::prelude::*;


#[derive(Debug, Default)]
pub struct InputMapperMessageHandler {
    pub mouse: MouseState,
    pub viewport_bounds: ViewportBounds
}

impl MessageHandler<InputMapperMessage, ()> for InputMapperMessageHandler {
    fn process_message(&mut self, message: InputMapperMessage, responses: &mut VecDeque<Message>, data: ()) {
        match message {
            InputMapperMessage::ViewportBounds(viewport_bounds) => {
                assert_eq!(viewport_bounds.len(), 1, "Only one viewport is currently supported");
                
                for bounds in viewport_bounds {
                    self.viewport_bounds = bounds;
                }
            }
            InputMapperMessage::DoubleClick(_) => todo!(),
            InputMapperMessage::PointerMove => todo!(),
            InputMapperMessage::WheelScroll => todo!(),
        }
    }
}