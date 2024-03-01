use tauri::{AppHandle, Manager};

use super::{error::ImageManagerError, image::DynImage};

pub struct ImageManager {
    images: Vec<Box<dyn DynImage>>,
    app: AppHandle,
}

impl ImageManager {
    pub fn new(app: AppHandle) -> Self {
        Self {
            images: Vec::new(),
            app
        }
    }

    pub fn delete(&self, idx: usize) -> Result<(), ImageManagerError> {
        self.images.get(idx).ok_or(ImageManagerError::ImageNotFound)?;
        Ok(())
    }

    fn emit_state(&self) {
    }
}
unsafe impl Send for ImageManager {}