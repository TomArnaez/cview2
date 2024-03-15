use std::{sync::Mutex, time::Duration};
use tauri::Manager;
use wrapper::{ExposureModes, SLImage};
use async_trait::async_trait;
use crate::capture::{capture::{CaptureSettings, CaptureStepOutput, JobInitOutput, StatefulCapture}, error::CaptureError, report::CaptureReportUpdate};

use super::{helpers::configure_device_for_capture, CaptureContext};

pub struct SequenceCapture {
    pub capture_settings: CaptureSettings,
    pub frame_count: usize,
    pub exposure_time: Duration,
    pub corrected: bool,
}

pub struct SequenceCaptureData {
    frames: Vec<SLImage>,
}

pub struct SequenceCaptureStep {
    frame: usize,
}

#[async_trait]
impl StatefulCapture for SequenceCapture {
    type Data = SequenceCaptureData;
    type Step = SequenceCaptureStep;
    type Result = Vec<SLImage>;

    const NAME: &'static str = "Sequence Capture";

    async fn init(
        &self,
        ctx: &CaptureContext
    ) -> Result<JobInitOutput<Self::Step, Self::Data>, CaptureError> {
        configure_device_for_capture(ctx.detector_handle.clone(), self.capture_settings).await?;

        let dims = ctx.detector_handle.get_image_dims().await?;
        let frames: Vec<SLImage> = (0..self.frame_count)
        .map(|_| SLImage::new(dims.0, dims.1))
        .collect();

        ctx.detector_handle
            .set_number_of_frames(self.frame_count as u32)
            .await?;
        ctx.detector_handle
            .set_exposure_mode(ExposureModes::SequenceMode)
            .await?;
        ctx.detector_handle
            .set_exposure_time(self.exposure_time)
            .await?;
        ctx.detector_handle.start_stream().await?;
        ctx.detector_handle.software_trigger().await?;

        Ok(JobInitOutput {
            steps: ((0..self.frame_count)
                .map(|frame| SequenceCaptureStep { frame })
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
    ) -> Result<CaptureStepOutput, CaptureError> {
        let dims = ctx.detector_handle.get_image_dims().await.unwrap();
        let vec = vec![0u16; (dims.0 * dims.1) as usize];
        ctx.detector_handle.acquire_image(vec, None).await.unwrap();

        let correction_images = ctx.correction_images.lock().await;
        let frame = &mut data.frames[step.frame];
        
        ctx.events_tx
            .send(CaptureReportUpdate::CompletedTaskCount(step.frame))
            .await
            .unwrap();
    }

    async fn finalise(&self, data: Self::Data, ctx: &CaptureContext) -> Self::Result {
        ctx.detector_handle.stop_stream().await.unwrap();
        data.frames
    }
}