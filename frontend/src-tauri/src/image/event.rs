use specta::Type;
use tauri_specta::Event;
use serde::Serialize;
use super::image::ImageDetails;

#[derive(Debug, Clone, Serialize, Type, Event)]
pub struct ImageManagerStateChanged(pub Vec<ImageDetails>);