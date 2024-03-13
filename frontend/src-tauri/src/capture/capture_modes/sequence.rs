use crate::capture::{
    capture::{CaptureSettings, JobInitOutput, StatefulCapture},
    detector::DetectorCaptureHandle,
    error::JobError,
    report::CaptureReportUpdate,
};
use std::time::Duration;
use tokio::sync::mpsc;
use wrapper::{ExposureModes, SLImage};

use super::helpers::configure_device_for_capture;

pub struct SequenceCapture {
    pub capture_settings: CaptureSettings,
    pub frame_count: usize,
    pub exposure_time: Duration,
}

pub struct SequenceCaptureData {
    frames: Vec<SLImage>,
    detector_handle: DetectorCaptureHandle,
}

pub struct SequenceCaptureStep {
    frame: usize,
}

#[async_trait::async_trait]
impl StatefulCapture for SequenceCapture {
    type Data = SequenceCaptureData;
    type Step = SequenceCaptureStep;
    type Result = Vec<SLImage>;

    const NAME: &'static str = "Sequence Capture";

    async fn init(
        &self,
        detector_handle: DetectorCaptureHandle,
    ) -> Result<JobInitOutput<Self::Step, Self::Data>, JobError> {
        configure_device_for_capture(detector_handle.clone(), self.capture_settings).await?;

        let dims = detector_handle.get_image_dims().await?;
        let frames: Vec<SLImage> = (0..self.frame_count)
        .map(|_| SLImage::new(dims.0, dims.1))
        .collect();

        detector_handle
            .set_number_of_frames(self.frame_count as u32)
            .await?;
        detector_handle
            .set_exposure_mode(ExposureModes::SequenceMode)
            .await?;
        detector_handle
            .set_exposure_time(self.exposure_time)
            .await?;
        detector_handle.start_stream().await?;
        detector_handle.software_trigger().await?;

        Ok(JobInitOutput {
            steps: ((0..self.frame_count)
                .map(|frame| SequenceCaptureStep { frame })
                .collect()),
            data: Self::Data {
                frames,
                detector_handle,
            },
        })
    }

    async fn execute_step(
        &self,
        step: &Self::Step,
        data: &mut Self::Data,
        events_tx: mpsc::Sender<CaptureReportUpdate>,
    ) {
        let dims = data.detector_handle.get_image_dims().await.unwrap();
        let vec = vec![0u16; (dims.0 * dims.1) as usize];
        data.detector_handle.acquire_image(vec, None).await.unwrap();

        events_tx
            .send(CaptureReportUpdate::CompletedTaskCount(step.frame))
            .await
            .unwrap();
    }

    async fn finalise(&self, data: Self::Data) -> Self::Result {
        data.detector_handle.stop_stream().await.unwrap();
        data.frames
    }
}
