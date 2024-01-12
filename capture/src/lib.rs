use async_stream::stream;

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

mod detector_controller;
mod image_handler;

enum StreamMessage {
    StreamProgress(StreamProgress),
    StreamError,
}

struct StreamProgress {
    curr_progress: u32,
    total_progress: u32,
    data: u32
}

trait Capture {
    fn run(&self) -> impl Stream<Item = StreamMessage>;
}

struct TestCapture {}

impl Capture for TestCapture {
    fn run(&self) -> impl Stream<Item = StreamMessage> {
        stream! {
            yield StreamMessage::StreamError;
        }
    }
}