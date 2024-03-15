use std::sync::{Arc, Mutex};
use serde::Serialize;
use tauri::AppHandle;
use uuid::Uuid;
use crate::shared_buffer::SharedBuffer;
use super::{error::ImageViewError, image::{ImageHandler, ImageVariant}};

struct StreamBuffer {
    buffer: Arc<Mutex<ImageHandler>>,
    callback: Box<dyn Fn(&ImageVariant) + Send + Sync>
}

impl StreamBuffer {
    fn update(&self) {
        let buffer = self.buffer.lock().unwrap();
        (self.callback)(&buffer.get_image());
    }
}

pub trait Viewable {
    fn register_callback(&mut self, callback: Box<dyn Fn(&ImageVariant) + Send + Sync>);
}

pub struct ImageViewController {
    views: Vec<ImageView>
}

impl ImageViewController {
    pub fn close_view(&mut self, id: ImageViewId) -> Result<(), ImageViewError> {
        todo!();
    }

    pub fn update_view_settings(&mut self, id: ImageViewId, settings: ImageViewSettings) -> Result<(), ImageViewError> {
        let view = self.get_view_mut(id)?;
        view.update_settings(settings);
        Ok(())
    }

    fn get_view(&self, id: ImageViewId) -> Result<&ImageView, ImageViewError> {
        self.views.iter().find(|i| i.id == id).map_or_else(
            || Err(ImageViewError::ImageViewNotFound(id)),
            |img| Ok(img)
        )    
    }

    fn get_view_mut(&mut self, id: ImageViewId) -> Result<&mut ImageView, ImageViewError> {
        self.views.iter_mut().find(|i| i.id == id).map_or_else(
            || Err(ImageViewError::ImageViewNotFound(id)),
            |img| Ok(img)
        )    
    }
}

type RGBA = [u8; 4];
pub type ImageViewId = Uuid;

#[derive(Debug, Serialize)]
pub struct SaturatedPixelSettings {
    saturated_limit: u16,
    saturated_colour: RGBA
}

#[derive(Debug, Serialize)]
pub struct ImageViewSettings {
    pub histogram_equilisation: bool,
    pub invert_colours: bool,
    pub saturation_settings: Option<SaturatedPixelSettings>
}

#[derive(Debug)]
pub struct ImageView {
    id: ImageViewId,
    shared_buffer: Arc<Mutex<SharedBuffer<u8>>>,
    settings: ImageViewSettings
}

impl ImageView {
    fn new(app: AppHandle, buffer_size: usize, settings: ImageViewSettings, view: &mut dyn Viewable) -> Self  {
        let shared_buffer =  Arc::new(Mutex::new(SharedBuffer::<u8>::new(buffer_size, app)));
        let buffer_clone = shared_buffer.clone();
        view.register_callback(Box::new(move |img| { 
            match img {
                ImageVariant::ImageU16(img) => {}
            }
        }));
        Self {
            id: ImageViewId::new_v4(),
            shared_buffer,
            settings
        }
    }

    fn update_settings(&mut self, settings: ImageViewSettings) {
        self.settings = settings;
    }
}

struct Brightness(u8);

impl Brightness {
    pub fn new(value: u8) -> Self {
        let clamped_value = value.min(100);
        Brightness(clamped_value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn set_value(&mut self, value: u8) {
        self.0 = value.min(100);
    }
}
