use tauri::State;
use tokio::sync::Mutex;

use super::{DetectorManager, TsDetector};

#[tauri::command]
pub async fn list_all_detectors(manager: State<'_, Mutex<DetectorManager>>) -> Result<Vec<TsDetector>, ()> {
    Ok(manager.lock().await.list_all_detectors().await)
}