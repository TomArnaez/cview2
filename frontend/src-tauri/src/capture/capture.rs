use std::{sync::Arc, time::Duration};
use specta::Type;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use tokio::sync::mpsc;
use wrapper::{ExposureModes, FullWellModes, SLBufferInfo, SLError, ROI};


use super::detector::DetectorCaptureHandle;

#[async_trait]
pub trait Capture: {
    async fn setup(&self, detector_handle: DetectorCaptureHandle, acquisition_settings: CaptureSettings) -> Result<(), SLError> {
        detector_handle.set_dds(acquisition_settings.dds_on).await?;
        detector_handle.set_full_well_mode(acquisition_settings.full_well_mode).await?;
        detector_handle.set_roi(acquisition_settings.roi).await?;
        detector_handle.set_test_mode(acquisition_settings.test_mode).await?;
        
        Ok(())
    }

    async fn run(&self, detector_handle: DetectorCaptureHandle, rx: mpsc::Receiver<CaptureCommand>) -> Result<mpsc::Receiver<CaptureResponse>, SLError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum CaptureMode {
    Stream(StreamCapture),
    Sequence(SequenceCapture),
}

#[derive(Debug)]
pub enum CaptureCommand {
    Cancel
}

#[derive(Debug, Clone, Serialize, Type)]
pub struct CaptureReport {
    capture: CaptureMode,
    frame: u32,
    buffer_info: SLBufferInfo
}

#[derive(Debug)]
pub enum CaptureResponse {
    Error(SLError),
    Report(CaptureReport),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
struct CaptureSettings {
    dds_on: bool,
    full_well_mode: FullWellModes,
    roi: ROI,
    test_mode: bool,
    timeout: Duration,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct StreamCapture {
    capture_settings: CaptureSettings,
    stream_time: Option<Duration>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct SequenceCapture {
    acquisition_settings: CaptureSettings,
    num_frames: u32,
    exposure_time: Duration
}

#[async_trait]
impl Capture for SequenceCapture {
    async fn run(&self, detector_handle: DetectorCaptureHandle, mut rx: mpsc::Receiver<CaptureCommand>) -> Result<mpsc::Receiver<CaptureResponse>, SLError> {
        self.setup(detector_handle.clone(), self.acquisition_settings).await?;
        let num_frames = self.num_frames;
        detector_handle.set_exposure_mode(ExposureModes::SequenceMode).await?;
        detector_handle.set_exposure_time(self.exposure_time).await?;
        detector_handle.set_number_of_frames(self.num_frames).await?;
        let (x, y) = detector_handle.get_image_dims().await?;
        //let images = SLImage::new_stack(x, y, self.num_frames);
        let timeout = self.acquisition_settings.timeout;

        let (acq_tx, acq_rx) = mpsc::channel(10);
        let capture = self.clone();
        tokio::spawn(async move {
            let mut frame = 0;
            let data = Arc::new(tokio::sync::Mutex::new(vec![0u16; (x * y) as usize]));
            while frame < num_frames {
                match detector_handle.acquire_image(Arc::clone(&data), Some(timeout)).await {
                    Ok(buffer_info) => {
                        acq_tx.send(CaptureResponse::Report(CaptureReport {
                            buffer_info,
                            capture: CaptureMode::Sequence(capture.clone()),
                            frame
                        })).await.unwrap();
                        frame += 1;
                    },
                    Err(e) => acq_tx.send(CaptureResponse::Error(e)).await.unwrap()
                }
            }
            detector_handle.stop_stream().await;
        });

        Ok(acq_rx)
    }
}

