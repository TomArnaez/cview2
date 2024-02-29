use std::{collections::VecDeque, time::Duration};
use wrapper::{ExposureModes, SLImage};
use crate::capture::{capture::{CaptureSettings, StatefulCapture}, detector::DetectorCaptureHandle, error::JobError};

use super::helpers::configure_device_for_capture;

pub struct SequenceCaptureInit {
    pub capture_settings: CaptureSettings,
    pub frame_count: usize,
    pub exposure_time: Duration
}

pub struct SequenceCaptureData {
    detector_handle: DetectorCaptureHandle,
    frames: Vec<SLImage>,
}

pub struct SequenceCaptureStep {
    frame: usize
}

#[async_trait::async_trait]
impl StatefulCapture for SequenceCaptureInit {
    type Data = SequenceCaptureData;
    type Step = SequenceCaptureStep;
    type Result = Vec<SLImage>;

    async fn init(&self, detector_handle: DetectorCaptureHandle) -> Result<VecDeque<Self::Step>, JobError> {
        configure_device_for_capture(detector_handle.clone(), self.capture_settings).await?;
        
        detector_handle.set_number_of_frames(self.frame_count as u32).await?;
        detector_handle.set_exposure_mode(ExposureModes::SequenceMode).await?;
        detector_handle.set_exposure_time(self.exposure_time).await?;
        detector_handle.start_stream().await?;
        detector_handle.software_trigger().await?;

        let data = Self::Data {
            detector_handle,
            frames: Vec::new(),
        };
       Ok((0..self.frame_count).map(|frame| SequenceCaptureStep { frame }).collect())
    }

    async fn execute_step(&self, step: &Self::Step, data: &mut Self::Data) {
        let image = &mut data.frames[step.frame];
    }

    async fn finalise(&self, data: Self::Data) -> Self::Result {
        data.frames
    }
}