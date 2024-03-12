use log::info;
use tauri::{AppHandle, Manager};

use crate::capture::DetectorStatus;

pub fn send(app: AppHandle, event: &Event) -> Result<(), ()> {
    info!("Sending event {:?}", event.name());
    app.emit(&event.name, &event.payload).unwrap();
    Ok(())
}

#[derive(Debug)]
pub struct Event {
    name: String,
    payload: serde_json::Value,
}

impl Event {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new_detector_connected() -> Event {
        Event {
            name: "new-detector-connected".to_owned(),
            payload: serde_json::json!("")
        }
    }

    pub fn detector_status_change(status: DetectorStatus) -> Event {
        Event {
            name: "detector-status-changed".to_owned(),
            payload: serde_json::json!(status)
        }
    }
}