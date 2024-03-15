use serde::Serialize;
use specta::Type;
use std::path::PathBuf;
use thiserror::Error;

use super::view::ImageViewId;

#[derive(Error, Serialize, Debug, Type)]
pub enum ImageManagerError {
    #[error("image not found error")]
    ImageNotFound,

    #[error("image file error")]
    ImageFileError(ImageFileError),
}

#[derive(Debug, Error, Serialize, Type)]
pub enum ImageFileError {
    #[error("couldn't open file at {0:?}")]
    CannotOpenFile(PathBuf),
    #[error("TIFF error")]
    TIFFError,
    #[error("Unsupported format")]
    UnsupportedFormat,
}

#[derive(Debug, Error, Serialize)]
pub enum ImageViewError {
    #[error("couldn't open image view with id {0:?}")]
    ImageViewNotFound(ImageViewId)
}