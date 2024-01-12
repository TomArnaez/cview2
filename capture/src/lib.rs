mod detector_controller;
mod capture_manager;

enum StreamMessage {
    StreamProgress(StreamProgress),
    StreamError,
}

struct StreamProgress {
    curr_progress: u32,
    total_progress: u32,
    data: u32
}