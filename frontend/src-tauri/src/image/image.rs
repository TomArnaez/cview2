use std::mem::size_of;
use image::{imageops, ColorType, ImageBuffer, Pixel, PixelWithColorType};
use serde::Serialize;
use specta::Type;
use tauri::AppHandle;
use uuid::Uuid;
use crate::shared_buffer::{HasTypeTag, SharedBuffer, TypeTag};
use super::image_commands::Command;

pub enum FlipDirection {
    X,
    Y
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct ImageDetails {
    id: Uuid,
    width: u32,
    height: u32,
    buffer_type: TypeTag
}

pub trait DynImage {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn execute_command(&mut self, command: Box<dyn Command>);
    fn get_colour_type(&self) -> ColorType;
    fn undo(&mut self);
    fn flip(&mut self, direction: FlipDirection);
    fn get_details(&self) -> ImageDetails;
}

pub struct ImageHandler<P: Pixel> 
where P::Subpixel : HasTypeTag
{
    pub data: ImageBuffer<P, SharedBuffer<P::Subpixel>>,
    commands: Vec<Box<dyn Command>>,
    id: Uuid,
}

impl<P: PixelWithColorType> DynImage for ImageHandler<P>
where
    P::Subpixel: HasTypeTag,
{
    fn width(&self) -> u32 {
        self.data.width()
    }

    fn height(&self) -> u32 {
        self.data.height()
    }

    fn get_colour_type(&self) -> ColorType {
        P::COLOR_TYPE
    }

    fn execute_command(&mut self, command: Box<dyn Command>) {
        command.execute(self);
        self.commands.push(command);
    }

    fn undo(&mut self) {
        
    }

    fn flip(&mut self, direction: FlipDirection) {
        match direction {
            FlipDirection::X => imageops::flip_horizontal_in_place(&mut self.data),
            FlipDirection::Y => imageops::flip_vertical_in_place(&mut self.data),
        }
    }

    fn get_details(&self) -> ImageDetails {
        ImageDetails {
            id: self.id,
            width: self.data.width(),
            height: self.data.height(),
            buffer_type: P::Subpixel::type_tag()
        }
    }
}

impl<P: PixelWithColorType> ImageHandler<P>
where P::Subpixel : HasTypeTag + 'static
{
    pub fn new(id: Uuid, buffer: SharedBuffer<P::Subpixel>, width: u32, height: u32, app: AppHandle) -> Self {
        let image_handler = Self {
            data: ImageBuffer::from_raw(width, height, buffer).unwrap(),
            id,
            commands: Vec::new()
        };

        image_handler
    }
}