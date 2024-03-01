use specta::Type;
use tauri_specta::Event;
use super::image::ImageDetails;

#[derive(Debug, Type, Event)]
pub struct ImageManagerStateChanged(Vec<ImageDetails>);