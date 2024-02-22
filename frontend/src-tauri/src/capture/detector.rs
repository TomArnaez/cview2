use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use log::info;
use tokio::sync::{mpsc, oneshot};
use wrapper::{scan_cameras, DeviceInterface, ExposureModes, FullWellModes, SLBufferInfo, SLDevice, SLDeviceInfo, SLError, SLImage, ROI};
use uuid::Uuid;

use super::capture::Capture;

const HEARTBEAT_PERIOD: Duration = Duration::from_millis(500);

enum DetectorMessage {
    AcquireImage(Arc<Mutex<Vec<u16>>>, Option<Duration>, oneshot::Sender<Result<SLBufferInfo, SLError>>),
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

struct DetectorActor {
    detector: SLDevice,
}

impl DetectorActor {
    async fn run(mut self, mut receiver: mpsc::Receiver<DetectorMessage>) {
        while let Some(message) = receiver.recv().await {
            match message {
                DetectorMessage::AcquireImage(buffer, timeout, sender) => sender.send(self.detector.acquire_image(buffer.lock().unwrap().as_mut_slice(), timeout)).unwrap(),
                DetectorMessage::GetImageDims(sender) => sender.send(self.detector.get_image_dims()).unwrap(),
                DetectorMessage::IsConnected(sender) => sender.send(self.detector.is_connected()).unwrap(), 
                DetectorMessage::OpenCamera(sender) => sender.send(self.detector.open_camera()).unwrap(),
                DetectorMessage::CloseCamera(sender) => sender.send(self.detector.close_camera()).unwrap(),
                DetectorMessage::SetDDS(dds_on, sender) => sender.send(self.detector.set_dds(dds_on)).unwrap(),
                DetectorMessage::SetFullWellMode(full_well_mode, sender) => sender.send(self.detector.set_full_well_mode(full_well_mode)).unwrap(),
                DetectorMessage::SetExposureTime(exposure_time, sender) => sender.send(self.detector.set_exposure_time(exposure_time)).unwrap(),
                DetectorMessage::SetROI(roi, sender) => sender.send(self.detector.set_roi(roi)).unwrap(),
                DetectorMessage::SetNumberOfFrames(frames, sender) => sender.send(self.detector.set_number_of_frames(frames)).unwrap(),
                DetectorMessage::SetExposureMode(exposure_mode, sender) => sender.send(self.detector.set_exposure_mode(exposure_mode)).unwrap(),
                DetectorMessage::SetTestMode(test_mode_on, sender) => sender.send(self.detector.set_test_mode(test_mode_on)).unwrap(),
                DetectorMessage::SoftwareTrigger(sender) => sender.send(self.detector.software_trigger()).unwrap(),
                DetectorMessage::StartStream(sender) => sender.send(self.detector.start_stream()).unwrap(),
                DetectorMessage::StopStream(sender) => sender.send(self.detector.stop_stream()).unwrap(),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DetectorHandle {
    sender: mpsc::Sender<DetectorMessage>
}

impl DetectorHandle {
    pub fn new_from_interface(interface: DeviceInterface) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let detector = DetectorActor { detector: SLDevice::new(interface).unwrap() };
        tokio::spawn(detector.run(receiver));

        Self { sender }
    }

    pub fn new_from_device_info(device_info: SLDeviceInfo) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let detector = DetectorActor { detector: SLDevice::new_from_device_info(device_info).unwrap() };
        tokio::spawn(detector.run(receiver));

        Self { sender }
    }

    pub async fn get_image_dims(&self) -> Result<(u32, u32), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::GetImageDims(resp_sender)).await;
        resp_receiver.await.expect("Actor died")
    }

    pub async fn is_connected(&self) -> bool {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::IsConnected(resp_sender)).await;
        resp_receiver.await.expect("Actor died")
    }

    pub async fn open_camera(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::OpenCamera(resp_sender)).await;
        resp_receiver.await.expect("Actor died")
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum DetectorStatus {
    Disconnected,
    Idle,
    Capturing
}

#[derive(Debug)]
pub struct DetectorInfo {
    image_dims: (u32, u32),
    interface: DeviceInterface
}

#[derive(Debug)]
pub struct DetectorControllerInner {
    detector_status: DetectorStatus,
    detector_info: Option<DetectorInfo>,
}

#[derive(Clone, Debug)]
pub struct DetectorCaptureHandle {
    sender: mpsc::Sender<DetectorMessage>
}

impl DetectorCaptureHandle {
    pub fn new(handle: DetectorHandle) -> Self {
        Self {
            sender: handle.sender.clone()
        }
    }

    pub async fn acquire_image(&self, buffer: Arc<Mutex<Vec<u16>>>, timeout: Option<Duration>) -> Result<SLBufferInfo, SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::AcquireImage(buffer, timeout, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn get_image_dims(&self) -> Result<(u32, u32), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::GetImageDims(resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_dds(&self, dds_on: bool) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SetDDS(dds_on, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_full_well_mode(&self, full_well_mode: FullWellModes) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SetFullWellMode(full_well_mode, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_roi(&self, roi: ROI) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SetROI(roi, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_exposure_mode(&self, exposure_mode: ExposureModes) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SetExposureMode(exposure_mode, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_exposure_time(&self, exposure_time: Duration) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SetExposureTime(exposure_time, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_number_of_frames(&self, num_frames: u32) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SetNumberOfFrames(num_frames, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn set_test_mode(&self, test_mode_on: bool) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SetTestMode(test_mode_on, resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn software_trigger(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::SoftwareTrigger(resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn start_stream(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::StartStream(resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn stop_stream(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::StopStream(resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }
}


#[derive(Debug)]
pub struct DetectorController {
    detector_handle: DetectorHandle,
    heartbeat_handle: tauri::async_runtime::JoinHandle<()>,
    inner: Arc<Mutex<DetectorControllerInner>>,
    status_tx: mpsc::Sender<DetectorStatus>,
}

impl DetectorController {
    pub async fn new_from_device_info(device_info: SLDeviceInfo, status_tx: mpsc::Sender<DetectorStatus>) -> DetectorController {
        info!("Initialising new detector for device_info:");
        let detector_handle = DetectorHandle::new_from_device_info(device_info.clone());
        DetectorController::setup(detector_handle, status_tx, device_info.device_interface).await
    }

    pub async fn new_from_interface(interface: DeviceInterface, status_tx: mpsc::Sender<DetectorStatus>) -> DetectorController {
        info!("Initialising new detector for interface:");
        let detector_handle = DetectorHandle::new_from_interface(interface);
        DetectorController::setup(detector_handle, status_tx, interface).await
    }

    async fn setup(detector_handle: DetectorHandle, status_tx: mpsc::Sender<DetectorStatus>, interface: DeviceInterface) -> DetectorController {
        let mut detector_status = DetectorStatus::Disconnected;
        let detector_info = detector_handle.open_camera().await
        .map(|_| async {
            detector_status = DetectorStatus::Idle;
            detector_handle.get_image_dims().await.map(|image_dims| DetectorInfo { image_dims, interface }).ok()
        }).unwrap().await;

        let inner = Arc::new(Mutex::new(DetectorControllerInner {
            detector_status,
            detector_info
        }));

        let heartbeat_handle = {
            let detector_handle = detector_handle.clone();
            let inner = inner.clone();
            let status_tx = status_tx.clone();
            tauri::async_runtime::spawn(async move {
                info!("Heartbeat thread started for {:?}", interface);
                loop {
                    // let mut inner_lock = inner.lock().unwrap();
                    // inner_lock.detector_status = DetectorStatus::Idle;
                    // match inner_lock.detector_status {
                    //     DetectorStatus::Disconnected => {
                    //         let detector_handle = detector_handle.clone();
                    //         if tauri::async_runtime::spawn_blocking(move || detector_handle.open_camera()).is_ok() {
                    //             inner_lock.detector_status = DetectorStatus::Idle;
                    //         }
                    //     },
                    //     _ => {
                    //         if !tauri::async_runtime::block_on(detector_handle.is_connected()) {
                    //             inner_lock.detector_status = DetectorStatus::Disconnected;
                    //         }
                    //     }
                    // }
                    // status_tx.blocking_send(inner_lock.detector_status.clone()).unwrap();
                    std::thread::sleep(HEARTBEAT_PERIOD)
                }
            })
        };

        DetectorController {
            detector_handle,
            heartbeat_handle,
            inner,
            status_tx
        }
    }

    pub fn run_capture(&self, capture: &dyn Capture) {
        let (tx, rx) = mpsc::channel(10);
        let detector_capture_handle = DetectorCaptureHandle::new(self.detector_handle.clone());
        capture.run(detector_capture_handle, rx);
    }
}

pub struct DetectorManager {
    detectors: HashMap<Uuid, DetectorController>
}

impl DetectorManager {
    pub async fn new() -> Self {
        let cameras = SLDevice::scan_cameras().unwrap();
        let mut detectors = HashMap::new();

        for device_info in cameras {
            let (tx, rx) = mpsc::channel(10);
            let detector_controller = DetectorController::new_from_device_info(device_info, tx).await;
            detectors.insert(Uuid::new_v4(), detector_controller);
        }

        DetectorManager {
            detectors
        }
    }

    pub fn get_detector_controller(&self, uuid: Uuid) -> Option<&DetectorController> {
        self.detectors.get(&uuid)
    }
}