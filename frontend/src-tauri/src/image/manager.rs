use std::path::PathBuf;
use std::fs::File;
use image::{Luma, PixelWithColorType};
use tauri::AppHandle;
use crate::shared_buffer::HasTypeTag;
use super::{error::{ImageFileError, ImageManagerError}, image::{ImageHandler, TsImage}, view::ImageView};
use tiff::decoder::{Decoder, DecodingResult};

pub struct ImageManager<'a> {
    images: Vec<ImageHandler>,
    views: Vec<ImageView<'a>>
}

impl<'a> ImageManager<'a> {
    pub fn new(app: AppHandle) -> Self {
        let manager = Self {
            images: Vec::new(),
            views: Vec::new()
        };
        manager.emit_state();
        manager 
    }

    pub fn list_all_images(&self) -> Vec<TsImage> {
        self.images.iter().map(|img| {
            TsImage {
                id: img.get_id(),
                width: img.get_width(),
                height: img.get_height()
            }
        }).collect()
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
        //self.images.push(Box::new(ImageHandler::<P>::new(Uuid::new_v4(), SharedBuffer::from((image, self.app.clone())), width, height, self.app.clone())));
        self.emit_state();
    }

    pub fn delete(&self, idx: usize) -> Result<(), ImageManagerError> {
        self.images.get(idx).ok_or(ImageManagerError::ImageNotFound)?;
        self.emit_state();
        Ok(())
    }

    fn emit_state(&self) {
        //self.app.emit("image-manager-state-changed", self.images.iter().map(|image| image.get_details()).collect::<Vec<ImageDetails>>()).unwrap();
        //ImageManagerStateChanged(self.images.iter().map(|image| image.get_details()).collect()).emit_all(&self.app).unwrap();
    }
}

unsafe impl<'a> Send for ImageManager<'a> {}