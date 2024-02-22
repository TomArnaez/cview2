use std::sync::Mutex;

use tauri::State;
use uuid::Uuid;

use super::{capture::CaptureMode, DetectorManager};

#[tauri::command]
#[specta::specta]
pub async fn run_capture(detector_manager: State<'_, Mutex<DetectorManager>>, detector_id: Uuid, capture: CaptureMode) -> Result<(), ()> {
    match capture {
        CaptureMode::Sequence(seq_capture) => {
            Ok(())
        },
        CaptureMode::Stream(stream_capture) => {
            Ok(())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn stop_capture(detector_manager: State<'_, Mutex<DetectorManager>>, detector_id: Uuid) -> Result<(), ()> {
    Ok(())
}