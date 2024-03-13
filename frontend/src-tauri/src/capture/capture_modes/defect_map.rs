use std::time::Duration;

pub struct DefectMapCapture {
    pub frames_per_capture: u32,
    pub exp_times: Vec<Duration>
}