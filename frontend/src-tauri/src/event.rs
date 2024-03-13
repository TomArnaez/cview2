use log::info;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::capture::{CaptureProgressEvent, DetectorStatus};

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

    pub fn detector_capture_progress(detector_id: Uuid, capture_progress_event: CaptureProgressEvent) -> Event {
        Event {
            name: format!("detector://{}//capture-progress", detector_id),
            payload: serde_json::json!(capture_progress_event)
        }
    }

    pub fn detector_status_change(detector_id: Uuid, status: DetectorStatus) -> Event {
        Event {
            name: format!("detector://{}/status-changed", detector_id),
            payload: serde_json::json!(status)
        }
    }

    pub fn new_detector_connected() -> Event {
        Event {
            name: "new-detector-connected".to_owned(),
            payload: serde_json::json!("")
        }
    }

}