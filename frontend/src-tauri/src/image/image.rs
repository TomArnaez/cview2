use image::{ImageBuffer, Pixel};
use uuid::Uuid;

use super::command::Command;
use crate::shared_buffer::SharedBuffer;

struct ImageHandler<P: Pixel> {
    data: ImageBuffer<P, SharedBuffer<P::Subpixel>>,
    history: Vec<Box<dyn Command>>,
    uuid: Uuid
}

impl<P: Pixel> ImageHandler<P> {
    pub fn new(width: u32, height: u32, buffer: SharedBuffer<P::Subpixel>) -> Self {
        Self {
            data: ImageBuffer::from_raw(width, height, buffer).unwrap(),
            history: Vec::new(),
            uuid: Uuid::new_v4()
        }
    }

    pub fn width(&self) -> u32 {
        self.data.width()
    }

    pub fn height(&self) -> u32 {
        self.data.height()
    }

    pub fn execute_command(&mut self, mut command: Box<dyn Command>) {
        command.execute();
        self.history.push(command);
    }

    pub fn undo_command(&mut self) {
        if let Some(mut command) = self.history.pop() {
            command.undo();
        }
    }
}