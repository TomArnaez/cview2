use std::path::PathBuf;
use std::fs::File;
use image::{Luma, PixelWithColorType};
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use crate::shared_buffer::{HasTypeTag, SharedBuffer};
use super::{error::{ImageFileError, ImageManagerError}, image::{DynImage, ImageDetails, ImageHandler}};
use tiff::decoder::{Decoder, DecodingResult};

pub struct ImageManager {
    images: Vec<Box<dyn DynImage>>,
    app: AppHandle,
}

impl ImageManager {
    pub fn new(app: AppHandle) -> Self {
        let manager = Self {
            images: Vec::new(),
            app
        };
        manager.emit_state();
        manager 
    }

    pub fn add_from_file(&mut self, path: PathBuf) -> Result<(), ImageManagerError> {
        let file = File::open(&path).map_err(|_| ImageManagerError::ImageFileError(ImageFileError::CannotOpenFile(path)))?;
        let mut decoder = Decoder::new(file).map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;
        let dims = decoder.dimensions().unwrap();
        let decoding_result = decoder.read_image().map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;
        match decoding_result {
            DecodingResult::U16(data) => {
                self.add_image::<Luma<u16>>(&data, dims.0, dims.1);
            },
            _ => return Err(ImageManagerError::ImageFileError(ImageFileError::UnsupportedFormat))
        };

        Ok(())
    }

    pub fn add_image<P>(&mut self, image: &[P::Subpixel], width: u32, height: u32) 
    where 
        P: PixelWithColorType + 'static,
        P::Subpixel: HasTypeTag + 'static
    {
        self.images.push(Box::new(ImageHandler::<P>::new(Uuid::new_v4(), SharedBuffer::from((image, self.app.clone())), width, height, self.app.clone())));
        self.emit_state();
    }

    pub fn delete(&self, idx: usize) -> Result<(), ImageManagerError> {
        self.images.get(idx).ok_or(ImageManagerError::ImageNotFound)?;
        self.emit_state();
        Ok(())
    }

    fn emit_state(&self) {
        self.app.emit("image-manager-state-changed", self.images.iter().map(|image| image.get_details()).collect::<Vec<ImageDetails>>()).unwrap();
        //ImageManagerStateChanged(self.images.iter().map(|image| image.get_details()).collect()).emit_all(&self.app).unwrap();
    }
}

unsafe impl Send for ImageManager {}