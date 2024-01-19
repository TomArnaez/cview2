use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex, Arc};
use std::time::{Instant, Duration};
use std::thread;

use async_stream::stream;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use wrapper::ffi::{ExposureModes, ROIinfo, DeviceInterface};
use wrapper::{SLDevice, SLError, SLImage};

const HEARTBEAT_PERIOD_MILLIS: u64 = 500;

pub enum DetectorEvent {
    EstablishedConnection(DetectorInfo),
    LostConnection,
}

#[derive(PartialEq, Debug)]
enum DetectorStatus {
    Disconnected,
    Idle,
    Capturing
}

#[derive(Debug, Copy, Clone)]
struct DetectorInfo {
    x_dim: u32,
    y_dim: u32,
    interface: DeviceInterface,
}

pub struct DetectorControllerInner {
    detector: SLDevice,
    detector_status: DetectorStatus,
    heartbeat_period: Duration,
    detector_info: Option<DetectorInfo>,
    event_tx: Sender<DetectorEvent>
}

pub struct DetectorController {
    inner: Arc<Mutex<DetectorControllerInner>>,
    heartbeat_handle: thread::JoinHandle<()>,
}

impl DetectorController {
    pub fn new() -> (DetectorController, mpsc::Receiver<DetectorEvent>) {
        let (tx, rx) = mpsc::channel();
        
        let inner = Arc::new(Mutex::new(DetectorControllerInner {
            detector: SLDevice::new(DeviceInterface::EIO_USB),
            detector_status: DetectorStatus::Idle,
            heartbeat_period: Duration::from_millis(HEARTBEAT_PERIOD_MILLIS),
            detector_info: None,
            event_tx: tx.clone()
        }));

        let inner_clone = Arc::clone(&inner);
        
        let heartbeat_handle = thread::spawn(move || {
            DetectorController::heartbeat(tx, inner_clone)
        });

       let controller =  DetectorController {
            inner,
            heartbeat_handle,
        };

        (controller, rx)
    }

    pub fn run_capture<T>(&self, cap: &dyn Capture<Output = T>) -> Result<T, SLError> {
        let inner_locked = self.inner.lock().unwrap();
        if inner_locked.detector_status == DetectorStatus::Idle {
            drop(inner_locked);
            return cap.capture(self.inner.clone());
        }

        Err(SLError::Busy)
    }

    fn heartbeat(rx: mpsc::Sender<DetectorEvent>, inner: Arc<Mutex<DetectorControllerInner>>) {
        loop {
            {
                let mut inner_locked = inner.lock().unwrap();
                if inner_locked.detector_status == DetectorStatus::Disconnected {
                    if let Ok(_) = inner_locked.detector.open_camera() {
                        let detector_info = DetectorInfo {
                            x_dim: inner_locked.detector.get_image_x_dim().unwrap(),
                            y_dim: inner_locked.detector.get_image_y_dim().unwrap(),
                            interface: DeviceInterface::EIO_USB
                        };

                        rx.send(DetectorEvent::EstablishedConnection(detector_info));

                        inner_locked.detector_info = Some(detector_info);
                        inner_locked.detector_status = DetectorStatus::Idle
                    } else {
                        println!("Failed to connect");
                    }
                } else {
                    if !inner_locked.detector.is_connected() {
                        rx.send(DetectorEvent::LostConnection);
                        
                        inner_locked.detector_status = DetectorStatus::Disconnected;
                        inner_locked.detector_info = None;
                    }
                    println!("We're connected!");
                }

                thread::sleep(inner_locked.heartbeat_period);
            }

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub enum CaptureMode {
    SequenceCapture(SequenceCapture),
    MultiCapture(MultiCapture),
    StreamCapture(StreamCapture)
}

#[derive(Debug, Clone, Deserialize, Serialize, specta::Type)]
struct CaptureSettings {
    exposure_time: Duration,
    #[serde(skip)]
    roi: Option<ROIinfo>,
    dds: bool,
}

impl CaptureSettings {
    pub fn builder() -> CaptureSettingsBuilder {
        CaptureSettingsBuilder::default()
    }
}

#[derive(Default)]
struct CaptureSettingsBuilder {
    exposure_time: Duration,
    dds: bool,
    roi: Option<ROIinfo>,
}

impl CaptureSettingsBuilder {
    pub fn new(exposure_time: Duration) -> CaptureSettingsBuilder {
        CaptureSettingsBuilder {
            exposure_time,
            dds: false,
            roi: None,
        }
    }

    pub fn dds(mut self, dds: bool) -> CaptureSettingsBuilder {
        self.dds =  dds;
        self
    }

    pub fn roi(mut self, roi: ROIinfo) -> CaptureSettingsBuilder {
        self.roi = Some(roi);
        self
    }

    pub fn build(self) -> CaptureSettings {
        CaptureSettings {
            exposure_time: self.exposure_time,
            dds: self.dds,
            roi: self.roi,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, specta::Type)]
pub struct SequenceCapture {
    capture_settings: CaptureSettings,
    frame_count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, specta::Type)]
pub struct MultiCapture {
    exposure_times: Vec<Duration>,
    frame_count: u32
}

#[derive(Debug, Clone, Deserialize, Serialize, specta::Type)]
pub struct StreamCapture {
    capture_settings: CaptureSettings,
    duration: Duration,
}

struct SequenceHandle {
    detector_inner: Arc<Mutex<DetectorControllerInner>>,
    is_active: Arc<AtomicBool>,
    frame_count: u32,
    total_frames: u32,
    stream: Pin<Box<dyn Stream<Item = SLImage> + Send>>,
}

struct MultiCaptureHandle {
    detector_inner: Arc<Mutex<DetectorControllerInner>>,
    is_active: Arc<AtomicBool>,
    multi_capture_settings: MultiCapture,
    current_capture_idx: u32,
    current_frame: u32,
    pub stream: Pin<Box<dyn Stream<Item = SLImage> + Send>>,
}

struct StreamHandle {
    detector_inner: Arc<Mutex<DetectorControllerInner>>,
    is_active: Arc<AtomicBool>,
    frame_count: u32,
    duration: Option<Duration>,
    start_time: Instant,
}

impl CaptureHandle for StreamHandle {
    fn cancel(&mut self) {
        self.is_active.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

trait Capture {
    type Output;

    fn pre_capture_setup(&self, detector: &mut DetectorControllerInner, capture_settings: &CaptureSettings) -> Result<(), SLError> {
        let detector = &mut detector.detector;
        detector.set_exposure_time(capture_settings.exposure_time)?;
        Ok(())
    }

    fn capture(&self, detector: Arc<Mutex<DetectorControllerInner>>) -> Result<Self::Output, SLError>;
}

pub trait CaptureHandle {
    fn cancel(&mut self);
}

impl StreamHandle {
    fn new(detector_inner: Arc<Mutex<DetectorControllerInner>>, duration: Option<Duration>) -> Self {
        StreamHandle {
            is_active: Arc::new(AtomicBool::new(true)),
            detector_inner,
            frame_count: 0,
            start_time: Instant::now(),
            duration
        }
    }
}

impl Capture for StreamCapture {
    type Output = StreamHandle;
    
    fn capture(&self, detector_inner: Arc<Mutex<DetectorControllerInner>>) -> Result<Self::Output, SLError> {
        let detector = &mut detector_inner.lock().unwrap().detector;
        detector.set_exposure_mode(ExposureModes::xfps_mode)?;
        detector.start_stream(self.capture_settings.exposure_time)?;

        Ok(StreamHandle::new(detector_inner.clone(), None))
    }
}

impl Capture for SequenceCapture {
    type Output = SequenceHandle;

    fn capture(&self, detector_inner: Arc<Mutex<DetectorControllerInner>>) -> Result<Self::Output, SLError> {
        let detector = &mut detector_inner.lock().unwrap().detector;
        detector.set_number_of_frames(self.frame_count)?;
        detector.set_exposure_time(self.capture_settings.exposure_time)?;
        detector.set_exposure_mode(ExposureModes::seq_mode)?;
        detector.go_live()?;
        detector.software_trigger()?;

        Ok(SequenceHandle::new(detector_inner.clone(), self.frame_count))
    }
}

impl Capture for MultiCapture {
    type Output = MultiCaptureHandle;

    fn capture(&self, detector_inner: Arc<Mutex<DetectorControllerInner>>) -> Result<Self::Output, SLError> {
        Ok(MultiCaptureHandle::new(detector_inner.clone(), self.clone()))
    }
}

impl SequenceHandle {
    fn new(detector_inner: Arc<Mutex<DetectorControllerInner>>, total_frames: u32) -> Self {
        let is_active = Arc::new(AtomicBool::new(true));

        let stream = 
        {
            let detector_inner = detector_inner.clone();
            let is_active = is_active.clone();
            stream! {
                for i in 0..total_frames {
                    if !is_active.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }

                    println!("getting frame {}", i);
                    let mut image;

                    {
                        let mut detector = detector_inner.lock().unwrap();
                        image = SLImage::new(detector.detector.get_image_x_dim().unwrap(), detector.detector.get_image_y_dim().unwrap());
                        while detector.detector.read_buffer(image.get_data_pointer(0), 0, 0).is_err() {
                        }
                    } 

                    yield image;
                }

                let mut detector = detector_inner.lock().unwrap();
                detector.detector.go_unlive();
                detector.detector_status = DetectorStatus::Idle;
            }
        };

        SequenceHandle {
            is_active,
            detector_inner,
            frame_count: 0,
            total_frames,
            stream: Box::pin(stream)
        }
    }
}

impl CaptureHandle for SequenceHandle {
    fn cancel(&mut self) {
        self.is_active.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

impl MultiCaptureHandle {
    fn new(detector_inner: Arc<Mutex<DetectorControllerInner>>, multi_capture_settings: MultiCapture) -> Self {
        let stream = 
        { 
            let detector_inner = detector_inner.clone();
            let multi_capture_settings = multi_capture_settings.clone();
            stream! {
                for exposure_time in multi_capture_settings.exposure_times {
                    let sequence_capture = SequenceCapture { capture_settings: CaptureSettings { dds: false, exposure_time, roi: None}, frame_count: multi_capture_settings.frame_count};
                    let sequence_handle = sequence_capture.capture(detector_inner.clone()).unwrap();
                    for await image in sequence_handle.stream {
                        yield image;
                    }
                }
            }
        };

        MultiCaptureHandle {
            is_active: Arc::new(AtomicBool::new(true)),
            detector_inner,
            multi_capture_settings,
            current_capture_idx: 0,
            current_frame: 0,
            stream: Box::pin(stream)
        }
    }
}

impl CaptureHandle for MultiCaptureHandle {
    fn cancel(&mut self) {
        self.is_active.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

struct TriggerCapture {
    exposure_time: u32,
}

struct TriggerHandle {
    detector_inner: Arc<Mutex<DetectorControllerInner>>,
    is_active: Arc<AtomicBool>,
    frame_count: u32,
}

impl TriggerHandle {
    fn send_trigger(&self) -> Result<(), SLError> {
        let detector = &mut self.detector_inner.lock().unwrap().detector;
        detector.software_trigger()
    }
}

impl CaptureHandle for TriggerHandle {
    fn cancel(&mut self) {
        self.is_active.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures_util::StreamExt;

    use crate::detector_controller::{DetectorController, MultiCapture, CaptureSettingsBuilder};

    #[tokio::test]
    async fn it_works() {
        let (detector_controller, _) = DetectorController::new();

        std::thread::sleep(Duration::from_secs(2));


        let capture_settings = CaptureSettingsBuilder::new(Duration::from_millis(100)).build();
        let multi = MultiCapture { exposure_times: vec![Duration::from_millis(100), Duration::from_millis(200)], frame_count: 10};

        let mut handle = detector_controller.run_capture(&multi).unwrap();
        while let Some(_) = handle.stream.next().await {
            println!("got image");
        }
    }
}
