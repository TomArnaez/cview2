use log::info;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_log::{Target, TargetKind};

mod image;
mod capture;
mod event;
mod shared_buffer;
mod task;

#[tauri::command]
#[specta::specta]
async fn init(app: AppHandle) {
    info!("Running init");

    app.manage(Mutex::new(capture::DetectorManager::new(app.clone()).await));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .setup(|app| {
            app.manage(Mutex::new(image::ImageViewController::new()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init,
            capture::commands::list_all_detectors,
            capture::commands::run_capture,
            image::commands::get_pixel_value,
            image::commands::list_all_views,
            image::commands::get_pixel_value,
            image::commands::open_image,
            // image::commands::save_image_as_bitmap,
            image::commands::save_as_tiff,
            image::commands::set_view_slice,
            image::commands::update_view_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
