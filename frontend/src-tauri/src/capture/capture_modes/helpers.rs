use std::{os::windows::process::CommandExt, path::PathBuf, process::Command};
use log::info;
use wrapper::SLError;

use crate::capture::{capture::CaptureSettings, detector::DetectorCaptureHandle};

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub async fn configure_device_for_capture(
    detector_handle: DetectorCaptureHandle,
    capture_settings: CaptureSettings,
) -> Result<(), SLError> {
    detector_handle.set_dds(capture_settings.dds).await?;
    detector_handle.set_full_well_mode(capture_settings.full_well_mode).await?;
    //detector_handle.set_roi(capture_settings.roi).await?;

    Ok(())
}

pub fn run_defect_map_gen(images_dir: &PathBuf, exe_path: &PathBuf) -> Result<PathBuf, ()> {
    let args = [images_dir.to_str().unwrap(), "1", "0", "-f", "-a", "-p"];
    if Command::new(exe_path).args(args).creation_flags(CREATE_NO_WINDOW).spawn().unwrap().wait().is_ok() {
        return Ok(images_dir.join("GlobalDefectMap.tif"));
    }
    Err(())
}