use std::{path::PathBuf, sync::Mutex};
use tauri::State;
use super::{error::ImageManagerError, ImageManager};

#[tauri::command]
#[specta::specta]
pub fn open_image(path: PathBuf, manager: State<Mutex<ImageManager>>) -> Result<(), ImageManagerError> {
    manager.lock().unwrap().add_from_file(path)
}

#[tauri::command]
#[specta::specta]
pub fn delete_image(idx: usize, manager: State<Mutex<ImageManager>>) -> Result<(), ImageManagerError> {
    manager.lock().unwrap().delete(idx)
}