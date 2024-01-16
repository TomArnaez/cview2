use serde::{Deserialize, Serialize};
use tauri::Manager;

use std::{cell::RefCell, sync::mpsc::Receiver};
use viewer::{application::Viewer, messages::{frontend::FrontendMessage, message::Message}};

thread_local! {
	static VIEWER: RefCell<Option<Viewer>> = RefCell::new(None);
    static FRONTEND_RX: RefCell<Option<Receiver<FrontendMessage>>> = RefCell::new(None);
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

fn init() {
    dispatch(Message::Init);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendEvent(FrontendMessage);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (viewer, rx) = Viewer::new();
    VIEWER.set(Some(viewer));
    FRONTEND_RX.set(Some(rx));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
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
