use std::time::Duration;
use async_trait::async_trait;
use wrapper::SLImage;
use crate::capture::{capture::{CaptureBuilder, CaptureSettings, JobInitOutput, StatefulCapture}, error::JobError};
use super::{sequence::SequenceCapture, CaptureContext};

pub struct SignalAccumulationCapture {
    pub capture_settings: CaptureSettings,
    pub exp_times: Duration,
    pub frame_count: usize
}

pub struct SignalAccumulationData {
    frames: Vec<SLImage>
}

pub struct SignalAccumulationStep {
    frame: usize
}

#[async_trait]
impl StatefulCapture for SignalAccumulationCapture {
    type Data = SignalAccumulationData;
    type Step = SignalAccumulationStep;
    type Result = Vec<SLImage>;

    const NAME: &'static str = "Signal Accumulation Capture";
    async fn init(
        &self,
        ctx: &CaptureContext
    ) -> Result<JobInitOutput<Self::Step, Self::Data>, JobError> {
        let dims = ctx.detector_handle.get_image_dims().await?;
        let frames: Vec<SLImage> = (0..self.frame_count)
        .map(|_| SLImage::new(dims.0, dims.1))
        .collect();
        
        Ok(JobInitOutput {
            steps: ((0..self.frame_count)
                .map(|frame| SignalAccumulationStep { frame })
                .collect()),
            data: Self::Data {
                frames,
            },
        })
    }

    async fn execute_step(
        &self,
        step: &Self::Step,
        data: &mut Self::Data,
        ctx: &CaptureContext
    ) {
        let sequence_capture = CaptureBuilder::new(SequenceCapture {
            capture_settings: self.capture_settings,
            exposure_time: self.exp_times,
            frame_count: self.frame_count,
            corrected: false
        }).build();
    }

    async fn finalise(&self, data: Self::Data, ctx: &CaptureContext) -> Self::Result {
        ctx.detector_handle.stop_stream().await.unwrap();
        data.frames
    }
}