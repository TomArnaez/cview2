use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_specta::{ts, Event};
use std::{cell::RefCell, thread};
use viewer::{application::Viewer, messages::{frontend::FrontendMessage, message::Message}};

thread_local! {
	static VIEWER: RefCell<Option<Viewer>> = RefCell::new(None);
}

fn dispatch(message: impl Into<Message>) {
    VIEWER.with(|viewer| {
        let mut viewer = viewer.borrow_mut();
        if let Some(ref mut v) = *viewer {
            v.handle_message(message)
        } else {
        }
    });
}

#[tauri::command]
fn init_after_frontend_ready() {
    dispatch(Message::Init);
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct FrontendEvent(FrontendMessage);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (viewer, rx) = Viewer::new();
    VIEWER.set(Some(viewer));

    let specta_builder = {
        let specta_builder = ts::builder().events(tauri_specta::collect_events![FrontendEvent]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let specta_builder = specta_builder.path("../src/communication/bindings.ts");

        specta_builder.into_plugin()
    };

    tauri::Builder::default()
        .plugin(specta_builder)
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();

            thread::spawn(move || {
                for recv in rx {
                    FrontendEvent(recv).emit_all(&handle).unwrap();
                }
            });
        
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            match event {
                _ => {},
            }
        })
}
