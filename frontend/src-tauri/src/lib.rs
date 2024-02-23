use std::sync::Mutex;
use log::info;
use tauri::{AppHandle, Manager};
use tauri_plugin_log::{Target, TargetKind};

mod capture;
mod image;
mod shared_buffer;

use crate::capture::DetectorManager;
use crate::shared_buffer::SharedBuffer;

#[tauri::command]
#[specta::specta]
async fn init(app: AppHandle) {
    info!("Running init");
    //app.manage(Mutex::new(DetectorManager::new().await));

    let shared_buffer = SharedBuffer::<u32>::new(100, app).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![init]);
        let specta_builder = specta_builder.path("../src/bindings.ts");
        specta_builder.into_plugin()
    };
    
    tauri::Builder::default()
        .plugin(specta_builder)
        .plugin(tauri_plugin_log::Builder::new().targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir { file_name: None }),
            Target::new(TargetKind::Webview),
        ]).build())
        .invoke_handler(tauri::generate_handler![init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}