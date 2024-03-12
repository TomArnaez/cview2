use image::{Luma, Rgba};
use tauri::AppHandle;
use crate::shared_buffer::SharedBuffer;
use super::image::ImageHandler;

pub struct ImageView<'a> {
    handler: &'a ImageHandler,
    brightness: Brightness,
    saturated_colour: Option<Rgba<u8>>,
    buffer: SharedBuffer<u8>
}

impl<'a> ImageView<'a> {
    fn new(handler: &'a ImageHandler, app: AppHandle) -> Self {
        let data: Vec<u8> = match handler.get_image() {
            super::image::ImageVariant::ImageU16(img) => {
                img.buffer.pixels().flat_map(|pixel| {
                    let Luma([luma]) = *pixel;
                    let value = (luma >> 8) as u8;
                    vec![value, value, value, 255]
                }).collect()
            },
        };
        Self {
            handler,
            brightness: Brightness::new(50),
            saturated_colour: None,
            buffer: SharedBuffer::from((data.as_slice(), app))
        }
    }
}

struct Brightness(u8);

impl Brightness {
    pub fn new(value: u8) -> Self {
        let clamped_value = value.min(100);
        Brightness(clamped_value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn set_value(&mut self, value: u8) {
        self.0 = value.min(100);
    }
}