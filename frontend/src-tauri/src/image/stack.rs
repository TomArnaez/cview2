use std::{sync::Arc};
use tiff::encoder::{colortype, TiffEncoder};
use uuid::Uuid;
use chrono::prelude::{DateTime, Utc};
use super::{error::ImageManagerError, image::{ImageProcessor, ImageVariant, TsImage}};

pub type ImageStackId = Uuid;

#[derive(Debug)]
pub struct ImageStack {
    id: ImageStackId,
    images: Vec<Arc<ImageVariant>>,
    timestamp: Option<DateTime<Utc>>,
    width: usize,
    height: usize
}

impl ImageStack {
    pub fn new(width: usize, height: usize, images: Vec<Arc<ImageVariant>>, timestamp: Option<DateTime<Utc>>) -> Self {
        let img = &images[0];
        Self {
            id: ImageStackId::new_v4(),
            images,
            timestamp,
            width,
            height
        }
    }

    pub fn images(&self) -> &Vec<Arc<ImageVariant>> {
        &self.images
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.images.len()
    }

    // pub fn get_processor(&self, slice: usize) -> Result<ImageProcessor, ()> {
    //     if slice >= self.depth() {
    //         return Err(())
    //     }

    //     Ok(ImageProcessor::new(self.images[slice]))
    // }

    // pub fn list_all_images(&self) -> Vec<TsImage> {
    //     self.images
    //         .iter()
    //         .map(|img| img.ts_image())
    //         .collect()
    // }

    pub fn get_image(&self, idx: usize) -> Option<&Arc<ImageVariant>> {
        self.images.get(idx)
    }

    // pub fn save_stack(&self, path: PathBuf) {
    //     let mut img_file = File::open(path).unwrap();
    //     let mut img_encoder = TiffEncoder::new(&mut img_file).unwrap();
        
    //     for handler in &self.images {
    //         let image = handler.buffer();
    //         match image.as_ref() {
    //             super::image::ImageVariant::ImageU16(data) => {
    //                 img_encoder.write_image::<colortype::Gray16>(handler.width() as u32, handler.height() as u32, data.buffer.as_raw()).unwrap();
    //             },
    //         }
    //     }
    // }
 
    // // TODO: error checking for image saving
    // pub fn save_image(&mut self, id: ImageId, path: PathBuf, format: ImageFormat) -> Result<(), ImageManagerError> {
    //     let image = self.get_image(id)?;
    //     image.save_image(path, format);
    //     Ok(())
    // }

    pub fn delete(&self, idx: usize) -> Result<(), ImageManagerError> {
        self.images
            .get(idx)
            .ok_or(ImageManagerError::ImageNotFound)?;
        Ok(())
    }

    pub fn id(&self) -> ImageStackId {
        self.id
    }
}

pub struct TsImageStack {
    id: ImageStackId,
    images: Vec<TsImage>,
    timestamp: Option<DateTime<Utc>>
}