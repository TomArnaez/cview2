use std::{sync::mpsc::Receiver, thread};

use capture::{DetectorController, DetectorEvent, CaptureHandle};

use crate::messages::prelude::*;

pub struct DetectorMessageHandler {
    detector_controller: DetectorController,
}

impl DetectorMessageHandler {
    pub fn new(responses: &mut Vec<FrontendMessage>) -> Self {
        let (detector_controller, rx) = DetectorController::new();

        Self {
            detector_controller,
        }
    }
}

impl MessageHandler<DetectorMessage, ()> for DetectorMessageHandler {
    fn process_message(&mut self, message: DetectorMessage, responses: &mut VecDeque<Message>, data: ()) {
        // match message {
        //     DetectorMessage::StartCapture(capture) => {
        //         match capture {
        //             capture::CaptureMode::MultiCapture(multi_capture) => {
        //                 match self.detector_controller.run_capture(&multi_capture) {
        //                     Ok(handler) => {
        //                     },
        //                     Err(_) => todo!(), 
        //                 }
        //             }
        //             capture::CaptureMode::SequenceCapture(_) => todo!(),
        //             capture::CaptureMode::StreamCapture(_) => todo!(),
        //         }
        //     },
        //     DetectorMessage::StopCapture => {

        //     },
        //     _  => {}
        // }
    }
}

pub fn handle_detector_events(rx: Receiver<DetectorEvent>, responses: &mut Vec<FrontendMessage>) {
    // for event in rx.iter() {
    //     match event {
    //         DetectorEvent::EstablishedConnection(detector_info) => {},
    //         DetectorEvent::LostConnection => {},
    //     }
    // }
}