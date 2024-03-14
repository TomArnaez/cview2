// use std::time::Duration;
// use async_trait::async_trait;

// use wrapper::SLImage;

// use crate::capture::{capture::{JobInitOutput, StatefulCapture}, error::JobError};

// use super::{helpers::configure_device_for_capture, CaptureContext};

// pub struct DarkMapCapture {
//     pub frame_count: u32,
//     pub exp_times: Vec<Duration>
// }

// pub struct DarkMapCaptureData {
//     images: Vec<Vec<SLImage>>,
// }

// pub struct DarkMapCaptureStep {
//     current_exposure_idx: usize,
// }

// #[async_trait]
// impl StatefulCapture for DarkMapCapture {
//     type Data = DarkMapCaptureData;
//     type Step = DarkMapCaptureStep;
//     type Result = Vec<SLImage>;

//     const NAME: &'static str = "Defect Map Capture";

//     async fn init(
//         &self,
//         ctx: &CaptureContext
//     ) -> Result<JobInitOutput<Self::Step, Self::Data>, JobError> {
//         let dims = ctx.detector_handle.get_image_dims().await.unwrap();
//         let full_data =
//         let frames: Vec<SLImage> = (0..self.frame_count)
//         .map(|_| SLImage::new(dims.0, dims.1))
//         .collect();
//     }

//     async fn execute_step(
//         &self,
//         step: &Self::Step,
//         data: &mut Self::Data,
//         ctx: &CaptureContext
//     ) {
//     }

//     async fn finalise(&self, data: Self::Data, ctx: &CaptureContext) -> Self::Result {
//     }
// }