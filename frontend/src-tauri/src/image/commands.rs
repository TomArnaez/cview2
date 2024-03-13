use super::{error::ImageManagerError, image::TsImage, manager::ImageId, ImageManager};
use std::{path::PathBuf, sync::Mutex};
use tauri::State;

#[tauri::command]
pub fn list_all_images(manager: State<Mutex<ImageManager>>) -> Vec<TsImage> {
    manager.lock().unwrap().list_all_images()
}

#[tauri::command]
pub fn save_image_as_tiff(
    manager: State<Mutex<ImageManager>>,
    path: PathBuf,
    id: ImageId,
) {
    manager.lock().unwrap().save_image(id, path, image::ImageFormat::Tiff);
}

#[tauri::command]
pub fn save_image_as_bitmap(
    manager: State<Mutex<ImageManager>>,
    path: PathBuf,
    id: ImageId,
) {
    manager.lock().unwrap().save_image(id, path, image::ImageFormat::Bmp);
}

#[tauri::command]
pub fn open_image(
    manager: State<Mutex<ImageManager>>,
    path: PathBuf,
) -> Result<(), ImageManagerError> {
    manager.lock().unwrap().add_from_file(path)
}

#[tauri::command]
pub fn delete_image(
    manager: State<Mutex<ImageManager>>,
    idx: usize,
) -> Result<(), ImageManagerError> {
    manager.lock().unwrap().delete(idx)
}
