use super::image::DynImage;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageManagerError {
    #[error("image not found error")]
    ImageNotFound
}

pub struct ImageManager {
    images: Vec<Box<dyn DynImage>>
}

impl ImageManager {
    pub fn new() -> Self {
        Self {
            images: Vec::new()
        }
    }

    pub fn delete(&self, idx: usize) -> Result<(), ImageManagerError> {
        self.images.get(idx).ok_or(ImageManagerError::ImageNotFound)?;
        Ok(())
    }
}
unsafe impl Send for ImageManager {}