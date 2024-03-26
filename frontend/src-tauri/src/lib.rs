use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use specta::ts::{BigIntExportBehavior, ExportConfig};

mod image;
mod capture;
mod event;
mod shared_buffer;
mod task;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .config(ExportConfig::default().bigint(BigIntExportBehavior::Number))
            .commands(tauri_specta::collect_commands![
                capture::commands::cancel_capture,
                capture::commands::get_detector,
                capture::commands::list_all_detectors,
                capture::commands::run_capture,
            ]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let specta_builder = specta_builder.path("../src/bindings.ts");
        specta_builder.into_plugin()
    };

    tauri::Builder::default()
        .plugin(specta_builder)
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
            app.manage(tokio::sync::Mutex::new(tauri::async_runtime::block_on(capture::DetectorController::new(app.handle().clone()))));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            capture::commands::cancel_capture,
            capture::commands::get_detector,
            capture::commands::list_all_detectors,
            capture::commands::run_capture,
            capture::commands::run_capture_chan,
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
