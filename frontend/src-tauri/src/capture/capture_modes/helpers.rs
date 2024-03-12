use wrapper::SLError;

use crate::capture::{capture::CaptureSettings, detector::DetectorCaptureHandle};

pub async fn configure_device_for_capture(detector_handle: DetectorCaptureHandle, capture_settings: CaptureSettings) -> Result<(), SLError> {
    detector_handle.set_dds(capture_settings.dds).await?;
    detector_handle.set_full_well_mode(capture_settings.full_well_mode).await?;
    detector_handle.set_roi(capture_settings.roi).await?;

    Ok(())
}