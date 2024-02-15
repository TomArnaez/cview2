use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use wrapper::{scan_cameras, DeviceInterface, ExposureModes, FullWellModes, SLBufferInfo, SLDevice, SLError, SLImage, ROI};
use uuid::Uuid;

const HEARTBEAT_PERIOD_MILLIS: u64 = 500;

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
    pub fn new(interface: DeviceInterface) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let detector = DetectorActor { detector: SLDevice::new(interface).unwrap() };
        tokio::spawn(detector.run(receiver));

        Self { sender }
    }

    pub async fn get_image_dims(&self) -> Result<(u32, u32), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::GetImageDims(resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn is_connected(&self) -> bool {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::IsConnected(resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
    }

    pub async fn open_camera(&self) -> Result<(), SLError> {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let _ = self.sender.send(DetectorMessage::OpenCamera(resp_sender)).await;
        resp_receiver.await.expect("Actor task has been killed")
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

#[derive(Debug)]
pub struct DetectorController {
    detector_handle: DetectorHandle,
    heartbeat_handle: tokio::task::JoinHandle<()>,
    inner: Arc<Mutex<DetectorControllerInner>>,
    status_tx: mpsc::Sender<DetectorStatus>,
}

impl DetectorController {
    pub async fn new(interface: DeviceInterface, status_tx: mpsc::Sender<DetectorStatus>) -> DetectorController {
        let detector_handle = DetectorHandle::new(interface);

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
            tokio::spawn(async move {
                loop {
                    let mut inner_lock = inner.lock().unwrap();
                    // match inner_lock.detector_status {
                    //     DetectorStatus::Disconnected => {
                    //         if detector_handle.open_camera().await.is_ok() {
                    //             inner_lock.detector_status = DetectorStatus::Idle;
                    //         }
                    //     },
                    //     _ => {
                    //         if !detector_handle.is_connected().await {
                    //             inner_lock.detector_status = DetectorStatus::Disconnected;
                    //         }
                    //     }
                    // }
                    
                    status_tx.send(inner_lock.detector_status.clone());
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

    // pub fn run_acquisition(&self, acquisition: Box<&dyn Acquisition>) -> Result<Pin<Box<dyn Stream<Item = SLImage>>>, ()> {
    //     if *self.detector_status.lock().unwrap() == DetectorStatus::Idle {
    //         let (tx, rx) = channel(8);
    //         Ok(acquisition.run(self.detector_handle.clone(), tx))
    //     }

    //     Err(())
    // }
}

#[derive(Clone, Debug)]
pub struct DetectorAcquisitionHandle {
    sender: mpsc::Sender<DetectorMessage>
}

impl DetectorAcquisitionHandle {
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct AcquistionSettings {
    dds_on: bool,
    full_well_mode: FullWellModes,
    roi: ROI,
    test_mode: bool,
    timeout: Duration,
}

#[derive(Debug)]
enum AcquisitionMessage {
    Error(SLError),
    Image(SLBufferInfo),
}

#[async_trait]
trait Acquisition: {
    async fn setup(&self, detector_handle: DetectorAcquisitionHandle, acquisition_settings: AcquistionSettings) -> Result<(), SLError> {
        detector_handle.set_dds(acquisition_settings.dds_on).await?;
        detector_handle.set_full_well_mode(acquisition_settings.full_well_mode).await?;
        detector_handle.set_roi(acquisition_settings.roi).await?;
        detector_handle.set_test_mode(acquisition_settings.test_mode).await?;
        
        Ok(())
    }

    async fn run(&self, detector_handle: DetectorAcquisitionHandle, rx: mpsc::Receiver<AcquisitionMessage>) -> Result<mpsc::Receiver<AcquisitionMessage>, SLError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamAcquisition {
    acquisition_settings: AcquistionSettings,
    stream_time: Option<Duration>,
}

// #[async_trait]
// impl Acquisition for StreamAcquisition {
//     async fn run(&self, detector_handle: DetectorHandle, mut rx: mpsc::Receiver<AcquisitionMessage>) -> Result<Pin<Box<dyn Stream<Item = AcquisitionMessage>>>, SLError> {
//         self.setup(detector_handle.clone(), self.acquisition_settings).await?;
//         detector_handle.set_exposure_mode(ExposureModes::XFPSMode).await?;
//         let (x, y) = detector_handle.get_image_dims().await?;
//         //let mut image = SLImage::new(x, y);
//         let stream = {
//             let data = Arc::new(Mutex::new(vec![0u16; (x * y) as usize]));
//             let timeout = self.acquisition_settings.timeout;
//             let detector_handle = detector_handle.clone();
//             stream! {
//                 let mut streaming_stop = false;
//                 while streaming_stop {
//                     while let Some(msg) = rx.recv().await {
//                     }
//                     match detector_handle.acquire_image(Arc::clone(&data), Some(timeout)).await {
//                         Err(e) => yield AcquisitionMessage::Error(e),
//                         Ok(buffer_info) => yield AcquisitionMessage::Image(buffer_info)
//                     }
//                 }
//             }.boxed()
//         };
//         Ok(stream)
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceAcquisition {
    acquisition_settings: AcquistionSettings,
    num_frames: u32,
    exposure_time: Duration
}

#[async_trait]
impl Acquisition for SequenceAcquisition {
    async fn run(&self, detector_handle: DetectorAcquisitionHandle, mut rx: mpsc::Receiver<AcquisitionMessage>) -> Result<mpsc::Receiver<AcquisitionMessage>, SLError> {
        self.setup(detector_handle.clone(), self.acquisition_settings).await?;
        let num_frames = self.num_frames;
        detector_handle.set_exposure_mode(ExposureModes::SequenceMode).await?;
        detector_handle.set_exposure_time(self.exposure_time).await?;
        detector_handle.set_number_of_frames(self.num_frames).await?;
        let (x, y) = detector_handle.get_image_dims().await?;
        //let images = SLImage::new_stack(x, y, self.num_frames);
        let timeout = self.acquisition_settings.timeout;

        let (acq_tx, acq_rx) = mpsc::channel(10);
        tokio::spawn(async move {
            let mut count = 0;
            let data = Arc::new(Mutex::new(vec![0u16; (x * y) as usize]));
            while count < num_frames {
                match detector_handle.acquire_image(Arc::clone(&data), Some(timeout)).await {
                    Ok(buffer_info) => {
                        acq_tx.send(AcquisitionMessage::Image(buffer_info)).await.unwrap();
                        count += 1;
                    },
                    Err(e) => acq_tx.send(AcquisitionMessage::Error(e)).await.unwrap()
                }
            }
            detector_handle.stop_stream().await;
        });

        Ok(acq_rx)
    }
}

// #[derive(Debug)]
// pub struct SoftwareTriggerAcquistion {
//     acquisition_settings: AcquistionSettings,
// }

// #[async_trait]
// impl Acquisition for SoftwareTriggerAcquistion {
//     async fn run(&self, detector_handle: DetectorHandle, mut rx: mpsc::Receiver<AcquisitionMessage>) -> Result<Pin<Box<dyn Stream<Item = AcquisitionMessage>>>, SLError> {
//         self.setup(detector_handle.clone(), self.acquisition_settings).await?;
//         detector_handle.set_exposure_mode(ExposureModes::TriggerMode).await?;
//         let (x, y) = detector_handle.get_image_dims().await?;
//         //let mut image = SLImage::new(x, y);
//         let stream = {
//             let data = Arc::new(Mutex::new(vec![0u16; (x * y) as usize]));
//             let timeout = self.acquisition_settings.timeout;
//             let detector_handle = detector_handle.clone();
//             stream! {
//                 let mut streaming_stop = false;
//                 while streaming_stop {
//                     while let Some(msg) = rx.recv().await {
//                         match msg {
//                             AcquisitionMessage::SoftwareTrigger => {
//                                 detector_handle.software_trigger().map_err(|err| {
//                                     streaming_stop = true;
//                                     yield AcquisitionMessage::Error(err);
//                                 });
//                             }
//                             AcquisitionMessage::Error(_) => todo!(),
//                             AcquisitionMessage::Image(_) => todo!(),
//                             AcquisitionMessage::Cancelled => todo!(),
//                             AcquisitionMessage::Completed => todo!(),
//                             AcquisitionMessage::Cancel => todo!(),
//                         }
//                     }
//                     match detector_handle.acquire_image(Arc::clone(&data), Some(timeout)).await {
//                         Err(e) => yield AcquisitionMessage::Error(e),
//                         Ok(buffer_info) => yield AcquisitionMessage::Image(buffer_info)
//                     }
//                 }
//             }.boxed()
//         };
//         Ok(stream)
//     }
// }


struct ExposureTime(Duration);
struct DarkMap {
    exposure_time: ExposureTime,
    dark_map: SLImage,
}
struct DefectMap(SLImage);
struct GainMap {
    exposure_time: ExposureTime,
    gain_map: SLImage,
}

struct DetectorCorrectionConfig {
    dark_maps: HashMap<ExposureTime, DarkMap>,
    gain_maps: HashMap<ExposureTime, GainMap>,
    defect_map: DefectMap,
}

pub struct DetectorManager {
    detectors: HashMap<Uuid, DetectorController>
}

impl DetectorManager {
    pub fn new() -> Self {
        let detector_infos = scan_cameras().unwrap();

        Self {
            detectors: HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {

}
