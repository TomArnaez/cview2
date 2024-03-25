use log::{error, info};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use std::time::Duration;
use tauri::{async_runtime::block_on, AppHandle};
use tokio::sync::{mpsc, oneshot, watch, Mutex};
use uuid::Uuid;
use wrapper::{
    Device, DeviceInterface, ExposureModes, FullWellModes, SLBufferInfo, SLDevice, SLDeviceInfo, SLError, SLImage, ROI
};

use crate::capture::error::CaptureError;
use crate::capture::report::CaptureStatus;
use crate::event::{self, Event};

use super::capture::StatefulCapture;
use super::capture_modes::CaptureContext;
use super::{
    capture::{Capture, CaptureCommand},
    error::DetectorControllerError,
    report::{CaptureReport, CaptureReportUpdate},
};

const HEARTBEAT_PERIOD: Duration = Duration::from_millis(500);

enum DetectorMessage {
    AcquireImage(
        Vec<u16>,
        Option<Duration>,
        oneshot::Sender<Result<(Vec<u16>, SLBufferInfo), SLError>>,
    ),
    CloseCamera(oneshot::Sender<Result<(), SLError>>),
    GetImageDims(oneshot::Sender<Result<(u32, u32), SLError>>),
    IsConnected(oneshot::Sender<bool>),
    OpenCamera(oneshot::Sender<Result<(), SLError>>),
    SetDDS(bool, oneshot::Sender<Result<(), SLError>>),
    SetFullWellMode(FullWellModes, oneshot::Sender<Result<(), SLError>>),
    SetROI(ROI, oneshot::Sender<Result<(), SLError>>),
    SetExposureMode(ExposureModes, oneshot::Sender<Result<(), SLError>>),
    SetExposureTime(Duration, oneshot::Sender<Result<(), SLError>>),
    SetTestMode(bool, oneshot::Sender<Result<(), SLError>>),
    SetNumberOfFrames(u32, oneshot::Sender<Result<(), SLError>>),
    SoftwareTrigger(oneshot::Sender<Result<(), SLError>>),
    StartStream(oneshot::Sender<Result<(), SLError>>),
    StopStream(oneshot::Sender<Result<(), SLError>>),
}

struct DetectorActor<T: Device> {
    detector: T,
}

impl<T: Device> DetectorActor<T> {
    async fn run(mut self, mut receiver: mpsc::Receiver<DetectorMessage>) {
        while let Some(message) = receiver.recv().await {
            match message {
                DetectorMessage::AcquireImage(mut buffer, timeout, sender) => {
                    let res = self.detector.acquire_image(buffer.as_mut_slice(), timeout);
                    match res {
                        Ok(buffer_info) => sender.send(Ok((buffer, buffer_info))).unwrap(),
                        Err(e) => sender.send(Err(e)).unwrap(),
                    }
                }
                DetectorMessage::GetImageDims(sender) => {
                    sender.send(self.detector.get_image_dims()).unwrap()
                }
                DetectorMessage::IsConnected(sender) => {
                    sender.send(self.detector.is_connected()).unwrap()
                }
                DetectorMessage::OpenCamera(sender) => {
                    sender.send(self.detector.open_camera()).unwrap()
                }
                DetectorMessage::CloseCamera(sender) => {
                    sender.send(self.detector.close_camera()).unwrap()
                }
                DetectorMessage::SetDDS(dds_on, sender) => {
                    sender.send(self.detector.set_dds(dds_on)).unwrap()
                }
                DetectorMessage::SetFullWellMode(full_well_mode, sender) => sender
                    .send(self.detector.set_full_well_mode(full_well_mode))
                    .unwrap(),
                DetectorMessage::SetExposureTime(exposure_time, sender) => sender
                    .send(self.detector.set_exposure_time(exposure_time))
                    .unwrap(),
                DetectorMessage::SetROI(roi, sender) => {
                    sender.send(self.detector.set_roi(roi)).unwrap()
                }
                DetectorMessage::SetNumberOfFrames(frames, sender) => sender
                    .send(self.detector.set_number_of_frames(frames))
                    .unwrap(),
                DetectorMessage::SetExposureMode(exposure_mode, sender) => sender
                    .send(self.detector.set_exposure_mode(exposure_mode))
                    .unwrap(),
                DetectorMessage::SetTestMode(test_mode_on, sender) => sender
                    .send(self.detector.set_test_mode(test_mode_on))
                    .unwrap(),
                DetectorMessage::SoftwareTrigger(sender) => {
                    sender.send(self.detector.software_trigger()).unwrap()
                }
                DetectorMessage::StartStream(sender) => {
                    sender.send(self.detector.start_stream()).unwrap()
                }
                DetectorMessage::StopStream(sender) => {
                    sender.send(self.detector.stop_stream()).unwrap()
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DetectorHandle {
    sender: mpsc::Sender<DetectorMessage>,
}

impl DetectorHandle {
    pub fn new_from_interface(interface: DeviceInterface) -> Self {
        DetectorHandle::setup(DetectorActor {
            detector: SLDevice::new(interface).unwrap(),
        })
    }

    pub fn new_from_device_info(device_info: SLDeviceInfo) -> Self {
        DetectorHandle::setup(DetectorActor {
            detector: SLDevice::new_from_device_info(device_info).unwrap(),
        })
    }

    fn setup<T: Device + 'static>(detector: DetectorActor<T>) -> DetectorHandle {
        let (sender, receiver) = mpsc::channel(8);
        std::thread::spawn(|| block_on(detector.run(receiver)));
        Self { sender }
    }

    pub async fn get_image_dims(&self) -> Result<(u32, u32), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::GetImageDims(resp_sender))
            .await;
        resp_receiver.await.expect("Actor died")
    }

    pub async fn is_connected(&self) -> bool {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::IsConnected(resp_sender))
            .await;
        resp_receiver.await.expect("Actor died")
    }

    pub async fn open_camera(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::OpenCamera(resp_sender))
            .await;
        resp_receiver.await.expect("Actor died")
    }
}

#[derive(Clone, Debug)]
pub struct DetectorCaptureHandle {
    sender: mpsc::Sender<DetectorMessage>,
}

impl DetectorCaptureHandle {
    pub fn new(handle: DetectorHandle) -> Self {
        Self {
            sender: handle.sender.clone(),
        }
    }

    pub async fn acquire_image(
        &self,
        buffer: Vec<u16>,
        timeout: Option<Duration>,
    ) -> Result<(Vec<u16>, SLBufferInfo), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::AcquireImage(buffer, timeout, resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn get_image_dims(&self) -> Result<(u32, u32), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::GetImageDims(resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_dds(&self, dds_on: bool) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SetDDS(dds_on, resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_full_well_mode(&self, full_well_mode: FullWellModes) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SetFullWellMode(
                full_well_mode,
                resp_sender,
            ))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_roi(&self, roi: ROI) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SetROI(roi, resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_exposure_mode(&self, exposure_mode: ExposureModes) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SetExposureMode(exposure_mode, resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_exposure_time(&self, exposure_time: Duration) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SetExposureTime(exposure_time, resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_number_of_frames(&self, num_frames: u32) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SetNumberOfFrames(num_frames, resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_test_mode(&self, test_mode_on: bool) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SetTestMode(test_mode_on, resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn software_trigger(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::SoftwareTrigger(resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn start_stream(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::StartStream(resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn stop_stream(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(DetectorMessage::StopStream(resp_sender))
            .await;
        resp_receiver.await.expect("Actor task has been killed")
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub enum DetectorStatus {
    Disconnected,
    Idle,
    Capturing,
}

#[derive(Debug)]
pub struct CorrectionImages {
    pub defect_map: Option<SLImage>,
    pub dark_maps: HashMap<Duration, SLImage>
}

impl Default for CorrectionImages {
    fn default() -> Self {
        CorrectionImages {
            defect_map: None,
            dark_maps: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct DetectorControllerInner {
    detector_status: DetectorStatus,
    capture_report: Option<CaptureReport>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectorSpecification {
    pub width: u32,
    pub height: u32,
}

// Typescript representation of detector state
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TsDetector {
    id: DetectorId,
    specification: DetectorSpecification,
    status: DetectorStatus,
    defect_map_available: bool,
    dark_map_exposures: Vec<Duration>
}

pub type DetectorId = Uuid;

#[derive(Debug)]
pub struct DetectorService {
    id: DetectorId,
    detector_handle: DetectorHandle,
    specification: DetectorSpecification,
    heartbeat_handle: tauri::async_runtime::JoinHandle<()>,
    inner: Arc<Mutex<DetectorControllerInner>>,
    status_tx: mpsc::Sender<DetectorStatus>,
    capture_cmd_tx: Option<watch::Sender<CaptureCommand>>,
    correction_images: Arc<Mutex<CorrectionImages>>
}

#[derive(Debug, Serialize)]
pub struct CaptureProgressEvent {
    pub task_count: usize,
    pub completed_task_count: usize,
    pub message: String,
}

impl DetectorService {
    pub async fn new_from_device_info(
        app: AppHandle,
        device_info: SLDeviceInfo,
        status_tx: mpsc::Sender<DetectorStatus>,
    ) -> Result<DetectorService, SLError> {
        info!(
            "Initialising new detector for device_info: {:?}",
            device_info
        );
        let detector_handle = DetectorHandle::new_from_device_info(device_info.clone());
        DetectorService::setup(
            app,
            detector_handle,
            status_tx,
            device_info.device_interface,
        )
        .await
    }

    pub async fn new_from_interface(
        app: AppHandle,
        interface: DeviceInterface,
        status_tx: mpsc::Sender<DetectorStatus>,
    ) -> Result<DetectorService, SLError> {
        info!("Initialising new detector for interface: {:?}", interface);
        let detector_handle = DetectorHandle::new_from_interface(interface);
        DetectorService::setup(app, detector_handle, status_tx, interface).await
    }

    async fn setup(
        app: AppHandle,
        detector_handle: DetectorHandle,
        status_tx: mpsc::Sender<DetectorStatus>,
        interface: DeviceInterface,
    ) -> Result<DetectorService, SLError> {
        let detector_status = DetectorStatus::Disconnected;
        detector_handle.open_camera().await?;

        let dims = detector_handle.get_image_dims().await?;
        let specification = DetectorSpecification {
            height: dims.0,
            width: dims.1,
        };

        let id = DetectorId::new_v4();

        event::send(
            app,
            &event::Event::detector_status_change(id, detector_status.clone()),
        )
        .unwrap();

        let inner = Arc::new(Mutex::new(DetectorControllerInner {
            detector_status,
            capture_report: None,
        }));

        let heartbeat_handle = {
            let detector_handle = detector_handle.clone();
            let inner = inner.clone();
            let status_tx = status_tx.clone();
            tauri::async_runtime::spawn(async move {
                info!("Heartbeat thread started for {:?}", interface);
                loop {
                    let mut inner_lock = inner.lock().await;
                    inner_lock.detector_status = DetectorStatus::Idle;
                    match inner_lock.detector_status {
                        DetectorStatus::Disconnected => {
                            let detector_handle = detector_handle.clone();
                            if detector_handle.open_camera().await.is_ok() {
                                inner_lock.detector_status = DetectorStatus::Idle;
                            }
                        }
                        _ => {
                            if !detector_handle.is_connected().await {
                                inner_lock.detector_status = DetectorStatus::Disconnected;
                            }
                        }
                    }
                    //status_tx.send(inner_lock.detector_status.clone()).await.unwrap();
                    std::thread::sleep(HEARTBEAT_PERIOD)
                }
            })
        };

        Ok(DetectorService {
            id,
            detector_handle,
            specification,
            heartbeat_handle,
            inner,
            status_tx,
            capture_cmd_tx: None,
            correction_images: Arc::new(Mutex::new(CorrectionImages::default()))
        })
    }

    pub async fn get_ts_detector(&self) -> TsDetector {
        let correction_images = self.correction_images.lock().await;

        TsDetector {
            id: self.id,
            specification: self.specification,
            status: self.inner.lock().await.detector_status,
            defect_map_available: correction_images.defect_map.is_some(),
            dark_map_exposures: correction_images.dark_maps.keys().cloned().collect()
        }
    }

    pub async fn run_capture<T: StatefulCapture>(
        &mut self,
        app: AppHandle,
        mut capture: Capture<T>,
    ) -> Result<(), DetectorControllerError> {
        let mut inner_lock = self.inner.lock().await;
        match inner_lock.detector_status {
            DetectorStatus::Disconnected => return Err(DetectorControllerError::DetectorDisconnected),
            DetectorStatus::Capturing => return Err(DetectorControllerError::CaptureInProgress),
            DetectorStatus::Idle => {
                inner_lock.detector_status = DetectorStatus::Capturing;
                let id = self.id;

                let (command_tx, command_rx) =
                    watch::channel::<CaptureCommand>(CaptureCommand::Cancel);
                let (events_tx, events_rx) = async_channel::unbounded();

                let ctx = CaptureContext {
                    app: app.clone(),
                    events_tx,
                    correction_images: self.correction_images.clone(),
                    detector_handle: DetectorCaptureHandle::new(self.detector_handle.clone())
                };

                tauri::async_runtime::spawn(async move {
                    let mut report = capture.report_mut().clone();

                    tokio::select! {
                        Ok(report_update) = events_rx.recv() => Self::handle_capture_progresss(app.clone(), id, &mut report, report_update),
                        capture_output = capture.run(ctx, command_rx) => {
                            match capture_output {
                                Ok(_) => report.status = CaptureStatus::Completed,
                                Err(CaptureError::Canceled) => report.status = CaptureStatus::Canceled,
                                Err(e) => {
                                    error!(
                                        "Job<id='{}', name='{}'> failed with error: {e:#?};",
                                        report.id, report.name
                                    );
                                    report.status = CaptureStatus::Failed;
                                }
                            }
                        }
                    }
                });

                self.capture_cmd_tx = Some(command_tx);

                Ok(())
            }
        }
    }

    pub async fn cancel_capture(&mut self) -> Result<(), DetectorControllerError> {
        let inner_lock = self.inner.lock().await;
        match inner_lock.detector_status {
            DetectorStatus::Capturing => {
                self.capture_cmd_tx
                    .as_mut()
                    .unwrap()
                    .send(CaptureCommand::Cancel)
                    .unwrap();
                Ok(())
            }
            _ => Err(DetectorControllerError::NoCaptureInProgress)
        }
    }

    fn handle_capture_progresss(
        app: AppHandle,
        id: DetectorId,
        report: &mut CaptureReport,
        update: CaptureReportUpdate,
    ) {
        match update {
            CaptureReportUpdate::TaskCount(task_count) => report.task_count = task_count,
            CaptureReportUpdate::CompletedTaskCount(completed_task_count) => report.completed_task_count = completed_task_count,
            CaptureReportUpdate::Message(message) => report.message = message
        }

        event::send(
            app,
            &&Event::detector_capture_progress(id, CaptureProgressEvent {
                completed_task_count: report.completed_task_count,
                task_count: report.task_count,
                message: "TBC".to_owned(),
            }),
        )
        .unwrap();
    }
}

pub struct DetectorManager {
    pub detectors: Vec<DetectorService>,
}

impl DetectorManager {
    pub async fn new(app: AppHandle) -> Self {
        let cameras = SLDevice::scan_cameras().unwrap();
        let mut detectors = Vec::new();

        for device_info in cameras {
            let (tx, rx) = mpsc::channel(10);
            match DetectorService::new_from_device_info(app.clone(), device_info, tx).await {
                Ok(detector) => {
                    detectors.push(detector);
                    event::send(app.clone(), &Event::new_detector_connected()).unwrap();
                }
                Err(e) => {
                    error!("Failed to connect to detector with error {:?}", e);
                }
            }
        }

        DetectorManager { detectors }
    }

    pub async fn run_capture<T: StatefulCapture>(&mut self, app: AppHandle, id: DetectorId, capture: Capture<T>) -> Result<(), DetectorControllerError> {
        let detector = self.get_detector_mut(id)?;
        detector.run_capture(app, capture).await?;
        Ok(())
    }

    pub async fn list_all_detectors(&self) -> Vec<TsDetector> {
        let detectors = self
            .detectors
            .iter()
            .map(|service| service.get_ts_detector());
        futures::future::join_all(detectors).await
    }

    fn get_detector_mut(&mut self, id: DetectorId) -> Result<&mut DetectorService, DetectorControllerError> {
        self.detectors.iter_mut().find(|det| det.id == id).map_or_else(
            || Err(DetectorControllerError::DetectorNotFound(id)),
            |det| Ok(det)
        )
    }
}

#[cfg(test)]
mod tests {

}
