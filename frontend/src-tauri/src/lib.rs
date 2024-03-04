use std::sync::Mutex;
use image::ImageManagerStateChanged;
use ::image::Luma;
use log::info;
use specta::ts::{BigIntExportBehavior, ExportConfig};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_log::{Target, TargetKind};

mod capture;
mod image;
mod shared_buffer;

use crate::{capture::DetectorManager, image::{delete_image, open_image, ImageManager}};

#[tauri::command]
#[specta::specta]
async fn init(app: AppHandle) {
    info!("Running init");
    app.manage(Mutex::new(DetectorManager::new().await));
    app.manage(Mutex::new(ImageManager::new(app.clone())));

}

#[tauri::command]
#[specta::specta]
fn test_cmd(image_manager: State<'_, Mutex<ImageManager>>) {
    let data = [0u16; 100];
    image_manager.lock().unwrap().add_image::<Luma<u16>>(&data, 100, 1);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![init, delete_image, open_image, test_cmd])
            .events(tauri_specta::collect_events![ImageManagerStateChanged]);
        let specta_builder = specta_builder.path("../src/bindings.ts");
        specta_builder.config(ExportConfig::new().bigint(BigIntExportBehavior::Number)).into_plugin()
    };
    
    tauri::Builder::default()
        .plugin(specta_builder)
        .plugin(tauri_plugin_log::Builder::new().targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir { file_name: None }),
            Target::new(TargetKind::Webview),
        ]).build())
        .invoke_handler(tauri::generate_handler![init, test_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}