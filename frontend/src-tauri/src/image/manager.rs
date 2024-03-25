// use super::{
//     error::{ImageFileError, ImageManagerError},
//     image::{ImageView, ImageU16, ImageVariant, TsImage}, stack::{ImageStack, ImageStackId},
// };
// use image::ImageBuffer;
use uuid::Uuid;
// use std::fs::File;
// use std::path::PathBuf;
// use tiff::decoder::{Decoder, DecodingResult};

pub type ImageId = Uuid;

// pub struct ImageManager {
//     images: Vec<ImageHandler>,
//     stacks: Vec<ImageStack>
// }

// impl ImageManager {
//     pub fn new() -> Self {
//         Self {
//             images: Vec::new(),
//             stacks: Vec::new(),
//         }
//     }

//     pub fn list_all_images(&self) -> Vec<TsImage> {
//         self.images
//             .iter()
//             .map(|img| img.ts_image())
//             .collect()
//     }

//     pub fn add_from_file(&mut self, path: PathBuf) -> Result<ImageStackId, ImageManagerError> {
//         let file = File::open(&path)
//             .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::CannotOpenFile(path)))?;
//         let mut decoder = Decoder::new(file)
//             .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;

//         let dims = decoder.dimensions().unwrap();
        
//         let mut images = Vec::new();
//         while decoder.next_image().is_ok() {
//             let decoding_result = decoder
//             .read_image()
//             .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;

//             match decoding_result {
//                 DecodingResult::U16(data) => {
//                     images.push(ImageHandler::new(ImageVariant::ImageU16(ImageU16 { buffer: ImageBuffer::from_raw(dims.0, dims.1, data).unwrap() })));
//                 }
//                 _ => {
//                     return Err(ImageManagerError::ImageFileError(
//                         ImageFileError::UnsupportedFormat,
//                     ))
//                 }
//             }
//         }

//         let image_stack = ImageStack::new(images, dims.0, dims.1, None);
//         let id: Uuid = image_stack.id();
//         self.stacks.push(image_stack);
//         Ok(id)
//     }

//     pub fn get_stack(&self, id: ImageId) -> Result<&ImageStack, ImageManagerError> {
//         self.stacks.iter().find(|i| i.id() == id)
//             .ok_or(ImageManagerError::ImageNotFound)
//     }
    
//     pub fn get_stack_mut(&mut self, id: ImageId) -> Result<&mut ImageStack, ImageManagerError> {
//         self.stacks.iter_mut().find(|i| i.id() == id)
//             .ok_or(ImageManagerError::ImageNotFound)
//     }

//     pub fn add_from_buffer(&mut self, image: ImageVariant) -> ImageId {
//         let image_handler = ImageHandler::new(image);
//         let id = image_handler.get_id();
//         self.images.push(image_handler);
//         id
//     }


// }

// unsafe impl Send for ImageManager {}

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;
//     use super::ImageManager;

//     #[test]
//     pub fn test_open_image() {
//         let mut manager = ImageManager::new();
//         manager.add_from_file(PathBuf::from("C:\\dev\\pathlib.tif")).unwrap();
//         let images = manager.list_all_images();
//         assert_eq!(images.len(), 1);
//         let image = images[0].clone();
//         assert_eq!(image.height, 500);
//         assert_eq!(image.width, 500)
//     }

//     // #[test]
//     // pub fn save_image_as_tiff() {
//     //     let mut manager = ImageManager::new();
//     //     manager.add_from_file(PathBuf::from("C:\\dev\\pathlib.tif")).unwrap();
//     //     let images = manager.list_all_images();
//     //     manager.save_image(images[0].id, PathBuf::from("C:\\dev\\testSave.tiff"), image::ImageFormat::Tiff).unwrap();
//     // }
// }