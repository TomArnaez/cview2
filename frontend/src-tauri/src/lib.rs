use std::sync::Mutex;
use log::info;
use tauri::{AppHandle, Manager};
use tauri_plugin_log::{Target, TargetKind};

mod capture;
mod event;
mod image;
mod shared_buffer;

use crate::{capture::{list_all_detectors, DetectorManager}, 
    image::{list_all_images, save_image_as_bitmap, save_image_as_tiff, ImageManager}};

#[tauri::command]
#[specta::specta]
async fn init(app: AppHandle) {
    info!("Running init");

    app.manage(Mutex::new(DetectorManager::new(app.clone()).await));
    app.manage(Mutex::new(ImageManager::new(app.clone())));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir { file_name: None }),
            Target::new(TargetKind::Webview),
        ]).build())
        .invoke_handler(tauri::generate_handler![init, 
            list_all_images,
            list_all_detectors,
            save_image_as_bitmap,
            save_image_as_tiff])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}