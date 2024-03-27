use tokio::sync::mpsc;
use log::info;
use tauri::{ipc::Channel, AppHandle, State};
use tokio::sync::Mutex;
use super::{capture::CaptureProgressEvent, capture_modes::SequenceCapture, error::DetectorControllerError, DetectorController, DetectorId, TsDetector};

#[tauri::command]
#[specta::specta]
pub async fn list_all_detectors(
    controller: State<'_, Mutex<DetectorController>>,
) -> Result<Vec<TsDetector>, ()> {
    info!("Received list_all_detectors command");
    Ok(controller.lock().await.list_all_detectors().await)
}

#[tauri::command]
#[specta::specta]
pub async fn get_detector(
    controller: State<'_, Mutex<DetectorController>>,
) -> Result<TsDetector, ()> {
    Ok(controller.lock().await.list_all_detectors().await[0].clone())
}

#[tauri::command]
pub async fn run_capture_chan(
    app: AppHandle, 
    controller: State<'_, Mutex<DetectorController>>, 
    id: DetectorId,
    capture: SequenceCapture,
    frontend_channel: Channel
) -> Result<(), DetectorControllerError> {
    info!("Received run_capture command <id={id}, capture={:?}>", capture);

    let (tx, mut rx) = mpsc::channel(10);
    controller.lock().await.run_capture(app, id, capture, tx).await?;

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CaptureProgressEvent::Error => {

                },
                CaptureProgressEvent::Completed(results) => {
                    println!("Got results of length {:?}", results.len());
                },
                CaptureProgressEvent::ProgressEvent(report) => {
                    frontend_channel.send(report);
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn run_capture(
    app: AppHandle, 
    controller: State<'_, Mutex<DetectorController>>, 
    id: DetectorId,
    capture: SequenceCapture,
) -> Result<(), DetectorControllerError> {
    info!("Received run_capture command <id={id}, capture={:?}>", capture);
    let (tx, rx) = mpsc::channel(10);
    controller.lock().await.run_capture(app, id, capture, tx).await
}

#[tauri::command]
#[specta::specta]
pub async fn cancel_capture(
    controller: State<'_, Mutex<DetectorController>>,
    id: DetectorId,
) -> Result<(), DetectorControllerError> {
    info!("Received cancel_capture command <id={id}>");
    controller.lock().await.cancel_capture(id).await
}