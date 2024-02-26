use std::{mem::size_of, sync::mpsc::Sender};
use image::{imageops, ImageBuffer, Pixel, PixelWithColorType};
use specta::Type;
use tauri::AppHandle;
use uuid::Uuid;

use crate::shared_buffer::{HasTypeTag, SharedBuffer};

use super::image_commands::Command;

#[derive(Copy, Clone)]
enum PixelType {
    U32,
    U16
}

trait PixelWrappedTrait {
    fn channels(&self);
}

impl<T: Pixel> PixelWrappedTrait for T {
    fn channels(&self) {
        self.channels();
    }
}

#[derive(Debug, Type)]
pub struct ImageReport {
    id: Uuid,
    width: u32,
    height: u32,
}

pub enum FlipDirection {
    X,
    Y
}

// Type erase the image pixel type
pub trait DynImage {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn execute_command(&mut self, command: Box<dyn Command>);
    fn undo(&mut self);
    fn flip(&mut self, direction: FlipDirection);
}

pub struct ImageHandler<P: Pixel> 
where P::Subpixel : HasTypeTag
{
    pub data: ImageBuffer<P, SharedBuffer<P::Subpixel>>,
    commands: Vec<Box<dyn Command>>,
    id: Uuid,
    tx: Sender<ImageReport>
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
}

impl<P: PixelWithColorType> ImageHandler<P>
where P::Subpixel : HasTypeTag + 'static
{
    pub fn new(id: Uuid, width: u32, height: u32, tx: Sender<ImageReport>, app: AppHandle) -> Self {
        let buffer = SharedBuffer::<P::Subpixel>::new((width * height) as usize * size_of::<P::Subpixel>(), app);

        let image_handler = Self {
            data: ImageBuffer::from_raw(width, height, buffer).unwrap(),
            id,
            tx,
            commands: Vec::new()
        };

        image_handler
    }
}