    use std::{fs::File, path::PathBuf, sync::Arc};
use image::ImageBuffer;
use tiff::{decoder::{Decoder, DecodingResult}, encoder::{colortype, TiffEncoder}};
use super::{error::{ImageFileError, ImageManagerError}, image::{ImageU16, ImageVariant}, stack::ImageStack, view::ImageView};

pub fn open_tiff(path: PathBuf) -> Result<ImageStack, ImageManagerError> {
    let file = File::open(&path)
        .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::CannotOpenFile(path)))?;
    let mut decoder = Decoder::new(file)
        .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;

    let (width, height) = decoder.dimensions().unwrap();
    let mut images = Vec::new();

    loop {
        let decoding_result = decoder
            .read_image()
            .map_err(|_| ImageManagerError::ImageFileError(ImageFileError::TIFFError))?;
        
        match decoding_result {
            DecodingResult::U16(data) => {
                images.push(Arc::new(ImageVariant::ImageU16(ImageU16 { buffer: ImageBuffer::from_raw(width, height, data).unwrap() })));
            }
            _ => {
                return Err(ImageManagerError::ImageFileError(
                    ImageFileError::UnsupportedFormat,
                ))
            }
        }

        if !decoder.more_images() {
            break;
        }
    }

    Ok(ImageStack::new(width as usize, height as usize, images, None))
}

pub fn save_tiff_stack(view: &ImageView, path: PathBuf) {
    let mut img_file = File::open(path).unwrap();
    let mut img_encoder = TiffEncoder::new(&mut img_file).unwrap();
    let stack = view.get_stack();

    for image in stack.images() {
        match image.as_ref() {
            super::image::ImageVariant::ImageU16(data) => {
                img_encoder.write_image::<colortype::Gray16>(stack.width() as u32, stack.height() as u32, data.buffer.as_raw()).unwrap();
            },
        }
    }
}

pub fn save_as_jpeg(view: &ImageView, slice: usize, path: PathBuf) -> Result<(), ()> {
    let stack = view.get_stack();
    if let Some(img) = stack.get_image(slice) {
        match img.as_ref() {
            super::image::ImageVariant::ImageU16(data) => {
                data.buffer.save_with_format(path, image::ImageFormat::Jpeg);
            },
        }
        Ok(())
    } else {
        Err(())
    }
}