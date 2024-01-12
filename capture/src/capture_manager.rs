use std::time::Duration;

use crate::detector_controller::DetectorController;

struct CaptureManager {
    detector_controller: DetectorController
}

impl CaptureManager {
    pub fn new() -> Self {
        let (controller, rx) = DetectorController::new();

        

        CaptureManager {
            detector_controller: controller
        }
    }
}



struct MultiCaptureSettings {
    exposure_times: Vec<Duration>
}

struct MultiCaptuerHandle {
}