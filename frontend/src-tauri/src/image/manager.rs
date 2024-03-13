use super::{
    error::{ImageFileError, ImageManagerError},
    image::{ImageHandler, ImageU16, ImageVariant, TsImage},
    view::ImageView,
};
use image::{ImageBuffer, ImageFormat};
use uuid::Uuid;
use std::fs::File;
use std::path::PathBuf;
use tiff::decoder::{Decoder, DecodingResult};

pub type ImageId = Uuid;

pub struct ImageManager<'a> {
    images: Vec<ImageHandler>,
    views: Vec<ImageView<'a>>,
}

impl<'a> ImageManager<'a> {
    pub fn new() -> Self {
        let manager = Self {
            images: Vec::new(),
            views: Vec::new(),
        };
        manager.emit_state();
        manager
    }

    pub fn list_all_images(&self) -> Vec<TsImage> {
        self.images
            .iter()
            .map(|img| img.get_ts_image())
            .collect()
    }

    pub fn add_from_file(&mut self, path: PathBuf) -> Result<(), ImageManagerError> {
        let file = File::open(&path)
            .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::CannotOpenFile(path)))?;
        let mut decoder = Decoder::new(file)
            .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;
        let dims = decoder.dimensions().unwrap();
        let decoding_result = decoder
            .read_image()
            .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;
        match decoding_result {
            DecodingResult::U16(data) => {
                self.add_from_buffer(ImageVariant::ImageU16(ImageU16 { buffer: ImageBuffer::from_raw(dims.0, dims.1, data).unwrap() }));
            }
            _ => {
                return Err(ImageManagerError::ImageFileError(
                    ImageFileError::UnsupportedFormat,
                ))
            }
        };

        Ok(())
    }

    pub fn add_from_buffer(&mut self, image: ImageVariant)
    {
        self.images.push(ImageHandler::new(image));
    }

    // TODO: error checking for image saving
    pub fn save_image(&mut self, id: ImageId, path: PathBuf, format: ImageFormat) -> Result<(), ImageManagerError> {
        let image = self.get_image(id)?;
        image.save_image(path, format);
        Ok(())
    }

    pub fn delete(&self, idx: usize) -> Result<(), ImageManagerError> {
        self.images
            .get(idx)
            .ok_or(ImageManagerError::ImageNotFound)?;
        self.emit_state();
        Ok(())
    }

    fn get_image(&self, id: ImageId) -> Result<&ImageHandler, ImageManagerError> {
        self.images.iter().find(|i| i.get_id() == id).map_or_else(
            || Err(ImageManagerError::ImageNotFound),
            |img| Ok(img)
        )
    }

    fn emit_state(&self) {
    }
}

unsafe impl<'a> Send for ImageManager<'a> {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::ImageManager;

    #[test]
    pub fn test_open_image() {
        let mut manager = ImageManager::new();
        manager.add_from_file(PathBuf::from("C:\\dev\\pathlib.tif")).unwrap();
        let images = manager.list_all_images();
        assert_eq!(images.len(), 1);
        let image = images[0];
        assert_eq!(image.height, 500);
        assert_eq!(image.width, 500)
    }

    #[test]
    pub fn save_image_as_tiff() {
        let mut manager = ImageManager::new();
        manager.add_from_file(PathBuf::from("C:\\dev\\pathlib.tif")).unwrap();
        let images = manager.list_all_images();
        manager.save_image(images[0].id, PathBuf::from("C:\\dev\\testSave.tiff"), image::ImageFormat::Tiff).unwrap();
    }
}