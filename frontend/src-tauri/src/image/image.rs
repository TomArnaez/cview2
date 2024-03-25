use std::{path::PathBuf, sync::Arc};
use image::{imageops::thumbnail, EncodableLayout, ImageBuffer, ImageFormat, Luma};
use serde::Serialize;
use tauri::image::Image;
use tokio::sync::broadcast;
use uuid::Uuid;

use super::{error::ImageManagerError, manager::ImageId, roi::{PixelIterable, Point, ROI}};

// Typescript representation of an image
#[derive(Debug, Clone, Serialize)]
pub struct TsImage {
    pub id: ImageId,
    pub width: usize,
    pub height: usize,
}
#[derive(Debug)]
pub enum ImageVariant {
    ImageU16(ImageU16),
}

#[derive(Debug)]
pub struct ImageU16 {
    pub buffer: ImageBuffer<Luma<u16>, Vec<u16>>,
}

pub struct ImageProcessor {
    id: ImageId,
    image: Arc<ImageVariant>,
    senders: Vec<broadcast::Sender<Arc<ImageVariant>>>
}

impl ImageProcessor {
    pub fn new(image: Arc<ImageVariant>) -> Self {
        Self {
            id: Uuid::new_v4(),
            image,
            senders: Vec::new()
        }
    }

    pub fn save_image(&self, path: PathBuf, format: ImageFormat) {
        match self.image.as_ref() {
            ImageVariant::ImageU16(img) => img.buffer.save_with_format(path, format).unwrap(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn buffer(&self) -> Arc<ImageVariant> {
        self.image.clone()
    }

    pub fn width(&self) -> usize {
        match self.image.as_ref() {
            ImageVariant::ImageU16(img) => img.buffer.width() as usize,
        }
    }

    pub fn height(&self) -> usize {
        match self.image.as_ref() {
            ImageVariant::ImageU16(img) => img.buffer.height() as usize,
        }
    }

    pub fn ts_image(&self) -> TsImage {
        TsImage {
            id: self.id,
            width: self.width(),
            height: self.height()
        }
    }

    // pub fn thumbnail(&self, width: usize, height: usize) -> Result<Image<'static>, ImageManagerError> {
    //     if width > self.height() || height < self.height() {
    //         return Err(ImageManagerError::OutOfBounds);
    //     }
    //     match self.image.as_ref() {
    //         ImageVariant::ImageU16(img) => {
    //             return Ok(Image::new_owned(thumbnail(&img.buffer, width, height).as_bytes().to_vec(), width, height));
    //         }
    //     }
    // }

    pub fn iter(&self, roi: ROI) -> ImageIterator {
        ImageIterator {
            image: self.image.clone(),
            coord_iterator: roi.pixels()
        }
    }
}

pub struct ImageIterator {
    image: Arc<ImageVariant>,
    coord_iterator: Box<dyn Iterator<Item=Point>>
}

impl ImageIterator {
    pub fn new(image: Arc<ImageVariant>, iterable: ROI) -> Self {
        Self {
            image,
            coord_iterator: iterable.pixels()
        }
    }
}

impl Iterator for ImageIterator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.coord_iterator.next() {
            match self.image.as_ref() {
                ImageVariant::ImageU16(img) => {
                    return Some(img.buffer.get_pixel(point.x, point.y)[0])
                }
            }
        }

        return None
    }
}

// impl Viewable for ImageHandler {
//     fn register_channel(&mut self, sender: broadcast::Sender<Arc<ImageVariant>>) {
//         sender.send(self.image.clone());
//         self.senders.push(sender);
//     }

//     fn image_size(&self) -> (usize, usize) {
//         (self.height() as usize, self.width() as usize)
//     }

// }