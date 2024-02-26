use std::collections::HashMap;
use std::sync::{Arc};
use std::time::Duration;
use log::info;
use serde::Serialize;
use specta::Type;
use tokio::sync::{mpsc, oneshot, Mutex};
use wrapper::{scan_cameras, DeviceInterface, ExposureModes, FullWellModes, SLBufferInfo, SLDevice, SLDeviceInfo, SLError, SLImage, ROI};
use uuid::Uuid;

use super::capture::{Capture, CaptureReport, CaptureResponse};

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
                DetectorMessage::AcquireImage(buffer, timeout, sender) => sender.send(self.detector.acquire_image(buffer.lock().await.as_mut_slice(), timeout)).unwrap(),
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
                    let mut inner_lock = inner.lock().await;
                    inner_lock.detector_status = DetectorStatus::Idle;
                    match inner_lock.detector_status {
                        DetectorStatus::Disconnected => {
                            let detector_handle = detector_handle.clone();
                            if detector_handle.open_camera().await.is_ok() {
                                inner_lock.detector_status = DetectorStatus::Idle;
                            }
                        },
                        _ => {
                            if !detector_handle.is_connected().await {
                                inner_lock.detector_status = DetectorStatus::Disconnected;
                            }
                        }
                    }
                    status_tx.send(inner_lock.detector_status.clone()).await.unwrap();
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

    pub async fn run_capture(&self, capture: &dyn Capture) -> Result<(), SLError> {
        let (tx, rx) = mpsc::channel(10);
        let detector_capture_handle = DetectorCaptureHandle::new(self.detector_handle.clone());
        let mut capture_rx = capture.run(detector_capture_handle, rx).await?;
        tauri::async_runtime::spawn(async move {
            while let Some(capture_res) = capture_rx.recv().await {
                println!("{:?}", capture_res);
            }
        });

        Ok(())
    }
}


#[derive(Debug, Clone, Serialize, Type)]
struct DetectorReport {
    uuid: Uuid,
    capture_report: Option<CaptureReport>
}

use futures::stream::SelectAll;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

pub struct DetectorManager {
    streams: Arc<Mutex<SelectAll<ReceiverStream<DetectorStatus>>>>,
}

impl DetectorManager {
    pub async fn new() -> Self {
        let cameras = SLDevice::scan_cameras().unwrap();
        // let mut detectors = HashMap::new();

        let mut streams = SelectAll::<ReceiverStream<DetectorStatus>>::new();

        for device_info in cameras {
            let (tx, rx) = mpsc::channel(10);
            let controller = DetectorController::new_from_device_info(device_info, tx).await;
            streams.push(ReceiverStream::new(rx));
        }

        let streams =  Arc::new(Mutex::new(streams));
        {
            let streams = streams.clone();
            tauri::async_runtime::spawn(async move {
                let mut streams = streams.lock().await;
                while let Some(message) = streams.next().await {
                    println!("Received: {:?}", message);
                    // Handle the message
                }
            });
        }

        DetectorManager {
            streams
        }

        // for device_info in cameras {
        //     let (tx, rx) = mpsc::channel(10);
        //     let controller = DetectorController::new_from_device_info(device_info, tx).await;
        //     let uuid = Uuid::new_v4();
        //     detectors.insert(uuid, DetectorState {
        //         uuid,
        //         controller,
        //         detector_rx: rx,
        //         capture_rx: None
        //     });
        // }

        // let detectors_mutex = Arc::new(std::sync::Mutex::new(detectors));

        // {
        //     let detector_mutex = detectors_mutex.clone();
        //     tokio::spawn(async move {
        //         let mut streams = Vec::new();
                
        //         // Lock the mutex and access the detectors HashMap
        //         let detectors = detector_mutex.lock().unwrap();
        //         for (_uuid, detector_state) in detectors.iter() {
        //             let detector_rx_stream = tokio_stream::wrappers::ReceiverStream::new(detector_state.detector_rx);
        //             streams.push(detector_rx_stream);
        //         }

        //     });
        // }

        // DetectorManager {
        //     detectors: detectors_mutex
        // }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::channel;

    use wrapper::DeviceInterface;

    use super::DetectorController;

    #[tokio::test]
    async fn test_controller() {
        let (tx, rx) = channel(10);
        let detector_controller = DetectorController::new_from_interface(DeviceInterface::EIO_USB, tx).await;

        loop {}
    }
}