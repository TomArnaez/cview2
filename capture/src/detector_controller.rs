use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex, Arc};
use std::task::{Poll, Context};
use std::time::Instant;
use std::{thread, time::Duration};

use async_stream::stream;
use futures_core::stream::BoxStream;
use wrapper::ffi::{ExposureModes, ROIinfo, DeviceInterface, BinningModes};
use wrapper::{SLDevice, SLError, SLImage, ffi};

const HEARTBEAT_PERIOD_MILLIS: u64 = 500;

pub enum DetectorEvent {
    EstablishedConnection,
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

struct DetectorControllerInner {
    detector: SLDevice,
    detector_status: DetectorStatus,
    heartbeat_period: Duration,
    detector_info: Option<DetectorInfo>,
    event_tx: Sender<DetectorEvent>
}

// Your controller struct
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
        inner.lock().unwrap().detector.open_camera();
        // loop {
        //     {
        //         let mut inner_locked = inner.lock().unwrap();
        //         if inner_locked.detector_status == DetectorStatus::Disconnected {
        //             if let Ok(_) = inner_locked.detector.open_camera() {
        //                 rx.send(DetectorEvent::EstablishedConnection);
        //                 inner_locked.detector_info = Some(DetectorInfo {
        //                     x_dim: inner_locked.detector.get_image_x_dim().unwrap(),
        //                     y_dim: inner_locked.detector.get_image_y_dim().unwrap(),
        //                     interface: DeviceInterface::EIO_USB
        //                 });

        //                 inner_locked.detector_status = DetectorStatus::Idle
        //             } else {
        //                 println!("Failed to connect");
        //             }
        //         } else {
        //             if !inner_locked.detector.is_connected() {
        //                 rx.send(DetectorEvent::LostConnection);
        //                 inner_locked.detector_status = DetectorStatus::Disconnected;
        //             }
        //             println!("We're connected!");
        //         }

        //         thread::sleep(inner_locked.heartbeat_period);
        //     }

        // }
    }
}

#[derive(Debug, Copy, Clone)]
struct CaptureSettings {
    exposure_time: Duration,
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

#[derive(Debug, Copy, Clone)]
struct SequenceCapture {
    capture_settings: CaptureSettings,
    frame_count: u32,
}

#[derive(Debug, Clone)]
struct MultiCapture {
    exposure_times: Vec<Duration>,
    frame_count: u32
}

#[derive(Debug, Clone)]
struct StreamCapture {
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

impl Stream for StreamHandle {
    type Item = SLImage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(duration) = self.duration {
            if Instant::now().duration_since(self.start_time) > duration {
                return Poll::Ready(None);
            }
        }

        let detector_inner_clone = Arc::clone(&self.detector_inner);
        let mut detector_lock = detector_inner_clone.lock().unwrap();

        let mut image = SLImage::new(detector_lock.detector.get_image_x_dim().unwrap(), detector_lock.detector.get_image_y_dim().unwrap());

        if detector_lock.detector.read_buffer(image.get_data_pointer(0), 0, 1000).is_ok() {
            drop(detector_lock);
            self.as_mut().get_mut().frame_count += 1;
            Poll::Ready(Some(SLImage::new(5, 5)))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl CaptureHandle for StreamHandle {
    fn cancel(&mut self) {
        self.is_active.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

trait Capture {
    type Output;

    fn capture(&self, detector: Arc<Mutex<DetectorControllerInner>>) -> Result<Self::Output, SLError>;
}

trait CaptureHandle {
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
        
        let detector_inner_clone = detector_inner.clone();
        let is_active_clone = Arc::clone(&is_active);
        let stream = stream! {
            for i in 0..total_frames {
                if !is_active_clone.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }

                println!("getting frame {}", i);
                let mut image = SLImage::new(2000, 2000);

                {
                    let mut detector = detector_inner_clone.lock().unwrap();
                    while detector.detector.read_buffer(image.get_data_pointer(0), 0, 0).is_err() {
                    }
                } 

                yield image;
            }

            let mut detector = detector_inner_clone.lock().unwrap();
            detector.detector.go_unlive();
            detector.detector_status = DetectorStatus::Idle;
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

use futures::stream::{self, Stream, StreamExt};

impl MultiCaptureHandle {
    fn new(detector_inner: Arc<Mutex<DetectorControllerInner>>, multi_capture_settings: MultiCapture) -> Self {
        let detector_inner_clone = detector_inner.clone();

        let multi_capture_settings_clone = multi_capture_settings.clone();

        let stream = stream! {
            for exposure_time in multi_capture_settings_clone.exposure_times {
                let sequence_capture = SequenceCapture { capture_settings: CaptureSettings { dds: false, exposure_time, roi: None}, frame_count: multi_capture_settings_clone.frame_count};
                let sequence_handle = sequence_capture.capture(detector_inner_clone.clone()).unwrap();
                for await image in sequence_handle.stream {
                    yield image;
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

impl Stream for TriggerHandle {
    type Item = SLImage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.is_active.load(std::sync::atomic::Ordering::Relaxed) {
            return Poll::Ready(None);
        }

        let detector_inner_clone = Arc::clone(&self.detector_inner);
        let mut detector_lock = detector_inner_clone.lock().unwrap();

        let mut image = SLImage::new(detector_lock.detector.get_image_x_dim().unwrap(), detector_lock.detector.get_image_y_dim().unwrap());

        if detector_lock.detector.read_buffer(image.get_data_pointer(0), 0, 1000).is_ok() {
            drop(detector_lock);
            self.as_mut().get_mut().frame_count += 1;
            Poll::Ready(Some(SLImage::new(5, 5)))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
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

    use futures_util::{pin_mut, StreamExt};

    use crate::detector_controller::{DetectorController, TriggerCapture, StreamCapture, SequenceCapture, CaptureHandle, CaptureSettings, MultiCapture, CaptureSettingsBuilder};

    #[tokio::test]
    async fn it_works() {
        let (detector_controller, rx) = DetectorController::new();

        std::thread::sleep(Duration::from_secs(2));


        let capture_settings = CaptureSettingsBuilder::new(Duration::from_millis(100)).build();
        let multi = MultiCapture { exposure_times: vec![Duration::from_millis(100), Duration::from_millis(200)], frame_count: 10};

        let seq = SequenceCapture { capture_settings, frame_count: 10};

        let mut handle = detector_controller.run_capture(&multi).unwrap();
        while let Some(image) = handle.stream.next().await {
            println!("got image");
        }
    }
}
