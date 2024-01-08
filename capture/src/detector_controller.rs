use std::pin::Pin;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Instant;
use std::{thread, time::Duration};

use async_stream::stream;
use futures_core::Stream;
use futures_util::StreamExt;
use wrapper::{SLDevice};

const HEARTBEAT_PERIOD_MILLIS: u64 = 200;

#[derive(PartialEq)]
enum DetectorStatus {
    Disconnected,
    Idle,
    Capturing
}

struct DetectorControllerInner {
    detector: SLDevice,
    detector_status: DetectorStatus,
    heartbeat_period: Duration,
}

// Your controller struct
struct DetectorController {
    inner: Arc<Mutex<DetectorControllerInner>>,
    heartbeat_handle: thread::JoinHandle<()>,
}

impl DetectorController {
    fn new() -> DetectorController {
        let (tx, rx) = mpsc::channel();
        let inner = Arc::new(Mutex::new(DetectorControllerInner {
            detector: SLDevice::new(),
            detector_status: DetectorStatus::Disconnected,
            heartbeat_period: Duration::from_millis(HEARTBEAT_PERIOD_MILLIS),
        }));

        let inner_clone = Arc::clone(&inner);
        let heartbeat_handle = thread::spawn(move || {
            DetectorController::heartbeat(rx, inner_clone)
        });

        DetectorController {
            inner,
            heartbeat_handle,
        }
    }

    fn run_capture<T>(&self, cap: &dyn Capture<Output = T>) -> Result<T, ()> {
        let mut inner_locked = self.inner.lock().unwrap();
        if inner_locked.detector_status != DetectorStatus::Idle {
            return Ok(cap.capture(self.inner.clone()));
        }

        Err(())
    }

    fn heartbeat(rx: mpsc::Receiver<()>, inner: Arc<Mutex<DetectorControllerInner>>) {
        loop {
            {
                let mut inner_locked = inner.lock().unwrap();
                if inner_locked.detector_status == DetectorStatus::Disconnected {
                    if let Ok(_) = inner_locked.detector.open_camera() {
                        println!("Opened Camera");
                        inner_locked.detector_status = DetectorStatus::Idle
                    }
                } else {
                    if !inner_locked.detector.is_connected() {
                        println!("Is Connected");
                        inner_locked.detector_status = DetectorStatus::Disconnected;
                    }
                }

                thread::sleep(inner_locked.heartbeat_period); // Or use inner_locked.heartbeat_period
            }

        }
    }
}

struct StreamCapture {
    duration: Duration
}

struct TriggerCapture {

}

struct StreamHandle {
    stream: Pin<Box<dyn Stream<Item = i32>>>

}

struct TriggerHandle {

}

trait Capture {
    type Output;

    fn capture(&self, detector: Arc<Mutex<DetectorControllerInner>>) -> Self::Output;
}

impl Capture for StreamCapture {
    type Output = StreamHandle;

    fn capture(&self, detector: Arc<Mutex<DetectorControllerInner>>) -> Self::Output {
        let mut inner_locked = detector.lock().unwrap();

        inner_locked.detector.start_stream(100);

        //let mut image = SLImage::new(inner_locked.detector.get_image_x_dim().unwrap(), inner_locked.detector.get_image_y_dim().unwrap());

        drop(inner_locked);

        let inner_clone = Arc::clone(&detector);
        return StreamHandle { stream: stream! {
            while true {
                let mut c: u16 = 5;
                let scoped = || {
                    let data_ptr: *mut u16 = &mut c as *mut u16;
                    let mut inner_locked = inner_clone.lock().unwrap(); 
                    //inner_locked.detector.read_frame(data_ptr);
                };
                yield 1;
            }
        }.boxed() };
    }
}

impl Capture for TriggerCapture {
    type Output = TriggerHandle;

    fn capture(&self, detector: Arc<Mutex<DetectorControllerInner>>) -> Self::Output {
        TriggerHandle {}
    }
}

impl StreamHandle {
    fn get_stream(&self) -> impl Stream<Item = i32> {
        //let image = SLImage::new(1401, height)
        stream! {
            for i in 0..3 {
                yield i
            }
        }
    }

    fn cancel(&self) {}
}

impl TriggerHandle {
    fn get_stream(&self) -> impl Stream<Item = i32> {
        stream! {
            for i in 0..3 {
                yield i
            }
        }
    }

    fn send_trigger(&self) {}

    fn cancel(&self) {}
}


#[cfg(test)]
mod tests {
    use futures_util::{pin_mut, StreamExt};

    use crate::detector_controller::{DetectorController, TriggerCapture};

    #[tokio::test]
    async fn it_works() {
        let detector_controller = DetectorController::new();

        loop {

        }
    }
}
