use std::{path::PathBuf, sync::Mutex};
use tauri::State;
use uuid::Uuid;
use super::{error::ImageManagerError, image::TsImage, ImageManager};

#[tauri::command]
pub fn list_all_images(manager: State<Mutex<ImageManager>>) -> Vec<TsImage> {
    manager.lock().unwrap().list_all_images()
}

#[tauri::command]
pub fn save_image_as_tiff(manager: State<Mutex<ImageManager>>, path: PathBuf, image_id: Uuid) -> Result<(), ()> {
    Ok(())
}

#[tauri::command]
pub fn save_image_as_bitmap(manager: State<Mutex<ImageManager>>, path: PathBuf, image_id: Uuid) -> Result<(), ()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn open_image(manager: State<Mutex<ImageManager>>, path: PathBuf) -> Result<(), ImageManagerError> {
    manager.lock().unwrap().add_from_file(path)
}

#[tauri::command]
#[specta::specta]
pub fn delete_image(manager: State<Mutex<ImageManager>>, idx: usize) -> Result<(), ImageManagerError> {
    manager.lock().unwrap().delete(idx)
}