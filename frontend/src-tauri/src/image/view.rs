use std::sync::Arc;
use log::info;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;
use crate::{capture::CaptureReport, shared_buffer::SharedBuffer};
use super::{error::ImageViewError, image::ImageVariant, stack::ImageStack};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TsImageView {
    id: ImageViewId,
    width: usize,
    height: usize,
    stack_size: usize,
    current_slice: usize,
    settings: ImageViewSettings,
}

pub struct ImageViewController {
    views: Vec<ImageView>
}

impl ImageViewController {
    pub fn new() -> Self {
        Self {
            views: Vec::new()
        }
    }

    pub fn add_view(&mut self, view: ImageView) {
        self.views.push(view);
    }

    pub fn list_all_views(&self) -> Vec<TsImageView> {
        self.views
            .iter()
            .map(|view| view.get_ts_view())
            .collect()
    }


    pub fn close_view(&mut self, id: ImageViewId) -> Result<(), ImageViewError> {
        if let Some(index) = self.views.iter().position(|view| view.id == id) {
            self.views.swap_remove(index);
            return Ok(())
        };
        Err(ImageViewError::ImageViewNotFound(id))
    }

    pub fn update_view_settings(&mut self, id: ImageViewId, settings: ImageViewSettings) -> Result<(), ImageViewError> {
        let view = self.get_view_mut(id)?;
        view.update_settings(settings);
        Ok(())
    }

    pub fn get_view(&self, id: ImageViewId) -> Result<&ImageView, ImageViewError> {
        self.views.iter().find(|i| i.id == id).map_or_else(
            || Err(ImageViewError::ImageViewNotFound(id)),
            |img| Ok(img)
        )    
    }

    pub fn get_view_mut(&mut self, id: ImageViewId) -> Result<&mut ImageView, ImageViewError> {
        self.views.iter_mut().find(|i| i.id == id).map_or_else(
            || Err(ImageViewError::ImageViewNotFound(id)),
            |img| Ok(img)
        )    
    }
}

type RGBA = [u8; 4];
pub type ImageViewId = Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaturatedPixelSettings {
    saturated_limit: u16,
    saturated_colour: RGBA
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageViewSettings {
    pub histogram_equilisation: bool,
    pub invert_colours: bool,
    pub saturated_pixel_settings: Option<SaturatedPixelSettings>,
}

impl Default for ImageViewSettings {
    fn default() -> Self {
        Self { 
            histogram_equilisation: false,
            invert_colours: false,
            saturated_pixel_settings: None
        }
    }
}

pub struct ImageView {
    id: ImageViewId,
    shared_buffer: SharedBuffer<u8>,
    stack: Arc<ImageStack>,
    current_slice: Arc<ImageVariant>,
    settings: ImageViewSettings,
    slice: usize,
    width: usize,
    height: usize,
    depth: usize,
}

impl ImageView {
    pub fn new(app: AppHandle, settings: ImageViewSettings, stack: Arc<ImageStack>) -> Self  {
        let width = stack.width();
        let height = stack.height();
        let depth = stack.depth();
        let current_slice = stack.get_image(0).unwrap().clone();

        let mut view = Self {
            id: ImageViewId::new_v4(),
            slice: 0,
            shared_buffer: SharedBuffer::<u8>::new((width * height * 4) as usize, app),
            stack,
            current_slice,
            settings,
            width,
            height,
            depth
        };
        view.update_shared_buffer();
        view
    }

    pub fn id(&self) -> ImageViewId {
        self.id
    }

    pub fn get_stack(&self) -> Arc<ImageStack> {
        self.stack.clone()
    }

    pub fn set_slice(&mut self, slice: usize) -> Result<(), ImageViewError> {
        if slice >= self.depth {
            Err(ImageViewError::SliceOutOfRange(slice))
        } else {
            self.slice = slice;
            self.current_slice = self.stack.images().get(slice).unwrap().clone();
            self.update_shared_buffer();
            Ok(())
        }
    }

    fn update_shared_buffer(&mut self) {
        let process_pixel = Self::create_pixel_processor(&self.settings);

        match self.current_slice.as_ref() {
            ImageVariant::ImageU16(img) => {
                for (x, y, pixel) in img.buffer.enumerate_pixels() {
                    let rgba = process_pixel(pixel[0]); // Process each pixel
                    let idx = (y * img.buffer.width() + x) as usize * 4;
                    self.shared_buffer[idx..idx + 4].copy_from_slice(&rgba);
                }
            },
        } 
    }

    fn create_pixel_processor(settings: &ImageViewSettings) -> Box<dyn Fn(u16) -> RGBA> {
        let invert = settings.invert_colours;
        let saturated_pixel_settings = settings.saturated_pixel_settings;
        
        Box::new(move |pixel: u16| -> RGBA {
            if let Some(settings) = &saturated_pixel_settings {
                if pixel > settings.saturated_limit {
                    return settings.saturated_colour;
                }
            }
            
            let val = if invert {
                u16::MAX - pixel
            } else {
                pixel
            };

            let val: u8 = (val as u32 * 255 / u16::MAX as u32) as u8;
            [val, val, val, 255]
        })
    }

    pub fn get_pixel_value(&self, x: u32, y: u32) -> Result<u32, ImageViewError> {
        match self.current_slice.as_ref() {
            ImageVariant::ImageU16(img) => {
                info!("{:?} {:?} {:?}", x, y, img.buffer.get_pixel(x, y));
                return Ok(img.buffer.get_pixel(x, y).0[0] as u32);
            },
        }
        Err(ImageViewError::NoImageAvailable)
    }

    pub fn get_ts_view(&self) -> TsImageView {
        TsImageView {
            id: self.id,
            width: self.width,
            height: self.height,
            stack_size: self.depth,
            current_slice: self.slice,
            settings: self.settings
        }
    }

    fn update_settings(&mut self, settings: ImageViewSettings) {
        self.settings = settings;
        self.update_shared_buffer();
    }
}