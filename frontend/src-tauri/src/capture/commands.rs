use tauri::{AppHandle, State};
use tokio::sync::Mutex;

use super::{error::DetectorControllerError, DetectorId, DetectorManager, TsDetector};

#[tauri::command]
pub async fn list_all_detectors(
    manager: State<'_, Mutex<DetectorManager>>,
) -> Result<Vec<TsDetector>, ()> {
    Ok(manager.lock().await.list_all_detectors().await)
}

#[tauri::command]
pub async fn run_capture(app: AppHandle, manager: State<'_, Mutex<DetectorManager>>, id: DetectorId) -> Result<(), ()> {
    //capture.id()?;
    //manager.lock().await.run_capture(app, id, capture).await?;
    Ok(())
}