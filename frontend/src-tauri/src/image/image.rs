use std::path::PathBuf;

use image::{ImageBuffer, ImageFormat, Luma};
use serde::Serialize;
use specta::Type;
use uuid::Uuid;

use super::manager::ImageId;

// Typescript representation of an image
#[derive(Debug, Clone, Copy, Serialize)]
pub struct TsImage {
    pub id: ImageId,
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
    id: ImageId,
    image: ImageVariant,
}

impl ImageHandler {
    pub fn new(image: ImageVariant) -> Self {
        Self {
            id: Uuid::new_v4(),
            image,
        }
    }

    pub fn save_image(&self, path: PathBuf, format: ImageFormat) {
        match &self.image {
            ImageVariant::ImageU16(img) => img.buffer.save_with_format(path, format).unwrap(),
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

    pub fn get_ts_image(&self) -> TsImage {
        TsImage {
            id: self.id,
            width: self.get_width(),
            height: self.get_height(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Type)]
pub struct ImageDetails {
    width: u32,
    height: u32,
}
