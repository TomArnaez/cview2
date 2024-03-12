use image::{ImageBuffer, Luma};
use serde::Serialize;
use specta::Type;
use uuid::Uuid;

// Typescript representation of an image
#[derive(Debug, Clone, Copy, Serialize)]
pub struct TsImage {
    pub id: Uuid,
    pub width: u32,
    pub height: u32,
}

pub enum ImageVariant {
    ImageU16(ImageU16),
}

pub struct ImageU16 {
    pub buffer: ImageBuffer<Luma<u16>, Vec<u16>>,
}

pub struct ImageU32 {
    buffer: ImageBuffer<Luma<u32>, Vec<u32>>,
}

pub struct ImageHandler {
    id: Uuid,
    image: ImageVariant,
}

impl ImageHandler {
    pub fn new(image: ImageVariant) -> Self {
        Self {
            id: Uuid::new_v4(),
            image
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_image(&self) -> &ImageVariant {
        &self.image
    }

    pub fn get_image_mut(&mut self) -> &mut ImageVariant {
        &mut self.image
    }

    pub fn get_width(&self) -> u32 {
        match &self.image {
            ImageVariant::ImageU16(img) => img.buffer.width(),
            //ImageVariant::ImageU32(img) => img.buffer.width(),
        }
    }

    pub fn get_height(&self) -> u32 {
        match &self.image {
            ImageVariant::ImageU16(img) => img.buffer.height(),
            //ImageVariant::ImageU32(img) => img.buffer.height(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Type)]
pub struct ImageDetails {
    width: u32,
    height: u32
}