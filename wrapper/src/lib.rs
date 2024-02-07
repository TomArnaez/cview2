use std::time::Duration;

use cxx::{UniquePtr, CxxString};

use ffi::{DeviceInterface, SLBufferInfo};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SLError {
    #[error("Operation completed successfully")]
    Success,

    #[error("Invalid parameter")]
    InvalidParam,

    #[error("No device found")]
    NoDevice,

    #[error("Item not found")]
    NotFound,

    #[error("Device or resource busy")]
    Busy,

    #[error("Operation timed out")]
    Timeout,

    #[error("Correction error")]
    Correction,

    #[error("Operation not supported")]
    NotSupported,

    #[error("Item already exists")]
    AlreadyExists,

    #[error("Internal error")]
    Internal,

    #[error("Other error")]
    Other,

    #[error("Device is closed")]
    DeviceClosed,

    #[error("Device is currently streaming")]
    DeviceStreaming,

    #[error("Configuration failed")]
    ConfigFailed,

    #[error("Configuration file not found")]
    ConfigFileNotFound,

    #[error("Not enough memory available")]
    NotEnoughMemory,

    #[error("Overflow error")]
    Overflow,

    #[error("Pipe error")]
    Pipe,

    #[error("Operation interrupted")]
    Interrupted,

    #[error("I/O error")]
    Io,

    #[error("Access error")]
    Access,

    #[error("Operation requires administrative privileges")]
    RequiresAdmin,

    #[error("Critical error occurred")]
    Critical,

    #[error("Unknown error")]
    Unknown,
}

impl SLError {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(SLError::Success),
            1 => Some(SLError::InvalidParam),
            2 => Some(SLError::NoDevice),
            3 => Some(SLError::NotFound),
            4 => Some(SLError::Busy),
            5 => Some(SLError::Timeout),
            6 => Some(SLError::Correction),
            7 => Some(SLError::NotSupported),
            8 => Some(SLError::AlreadyExists),
            9 => Some(SLError::Internal),
            10 => Some(SLError::Other),
            11 => Some(SLError::DeviceClosed),
            12 => Some(SLError::DeviceStreaming),
            13 => Some(SLError::ConfigFailed),
            14 => Some(SLError::ConfigFileNotFound),
            15 => Some(SLError::NotEnoughMemory),
            16 => Some(SLError::Overflow),
            17 => Some(SLError::Pipe),
            18 => Some(SLError::Interrupted),
            19 => Some(SLError::Io),
            20 => Some(SLError::Access),
            21 => Some(SLError::RequiresAdmin),
            22 => Some(SLError::Critical),
            _ => None,
        }
    }
}

fn get_error(err_code: i32) -> Result<(), SLError> {
    let err = SLError::from_i32(err_code).unwrap();
    match err {
        SLError::Success => Ok(()),
        _ => Err(err)
    }
}

#[cxx::bridge]
pub mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    pub enum ExposureModes {
        seq_mode = 1,
		fps25_mode,
		fps30_mode,
		trig_mode,
		xfps_mode
    }

    #[repr(i32)]
    #[derive(Debug)]
    pub enum DeviceInterface {
		CL = 0,
		USB = 1,
		PLEORA = 3,
		S2I_GIGE = 4,
		EIO_USB = 5,
		UNKNOWN = 6
    }

    #[repr(i32)]
    #[derive(Debug)]
    pub enum ModelInterface {
		CL,
		USB,
		GIGE,
		CXP,
		CLF,
		CLB,
		S2I_GIGE
    }

    #[derive(Debug, Copy, Clone)]
    struct ROIinfo {
        X: i32,
        Y: i32,
        W: i32,
        H: i32
    }

    #[derive(Debug, Copy, Clone)]
    struct SLBufferInfo {
        width: i32,
        height: i32,
    }

    #[repr(i32)]
    #[derive(Debug, Copy, Clone)]
    pub enum BinningModes
	{
		x11 = 1,
		x22 = 2,
		x44 = 3,
		BinningUnknown = 4,
	}

    /*
    pub struct SLDeviceInfo<'a> {
        interface: DeviceInterface,
        detector_ip_addresss: &'a CxxString,
        ip: &'a CxxString,
        int: i32,
        params: &'a CxxString,
        force_ip: &'a CxxString,
        log_file_path: &'a CxxString
    }


    #[derive(Debug)]
    pub struct ModelInfo<'a> {
        set: bool,
        code: &'a CxxString,
        full_code: &'a CxxString,
        type_name: &'a CxxString,    
        hash: &'a CxxString,
        model: &'a CxxString,
        configuration: &'a CxxString,
        device_width: i32,
        device_height: i32,
        num_temperature_sensors: i32,
        model_interface: ModelInterface,
        pixel_size: i32
    }
    */


    unsafe extern "C++" {
        include!("C:\\dev\\repos\\cview2\\wrapper\\include\\wrapper.h");

        type SLDevice;
        type ExposureModes;
        type DeviceInterface;
        type ROIinfo;
        type ModelInfo;
        type ModelInterface;
        type BinningModes;
        type SLBufferInfo;
        
        fn new_sl_device(device_interface: DeviceInterface) -> UniquePtr<SLDevice>;
        fn open_camera(device: Pin<&mut SLDevice>) -> i32;
        fn close_camera(device: Pin<&mut SLDevice>) -> i32;
        fn start_stream(device: Pin<&mut SLDevice>) -> i32;
        fn start_stream_exp_time(device: Pin<&mut SLDevice>, exp_time_ms: i32) -> i32;
        fn stop_stream(device: Pin<&mut SLDevice>) -> i32;
        fn acquire_image(device: Pin<&mut SLDevice>, buffer: &mut [u16]) -> SLBufferInfo;
        fn software_trigger(device: Pin<&mut SLDevice>) -> i32;
        fn is_connected(device: Pin<&mut SLDevice>) -> bool;
        fn set_exposure_time(device: Pin<&mut SLDevice>, exp_time_ms: i32) -> i32;
        fn set_exposure_mode(device: Pin<&mut SLDevice>, exposure_mode: ExposureModes) -> i32;
        fn set_number_of_frames(device: Pin<&mut SLDevice>, exp_time_ms: i32) -> i32;
        fn get_image_x_dim(device: Pin<&mut SLDevice>) -> i32;
        fn get_image_y_dim(device: Pin<&mut SLDevice>) -> i32;
        // fn set_roi(device: Pin<&mut SLDevice>, roi: &mut ROIinfo) -> i32;
        // fn get_roi(device: Pin<&mut SLDevice>, roi: &mut ROIinfo) -> i32;
        //fn get_model_info(device: Pin<&mut SLDevice>) -> ModelInfo;

        type SLImage;
        
        fn new_sl_image(width: i32, height: i32) -> UniquePtr<SLImage>;
        fn read_tiff_image(filename: &CxxString, image: Pin<&mut SLImage>) -> bool;
        fn write_tiff_image(filename: &CxxString, image: Pin<&mut SLImage>, bits: i32) -> bool;
        unsafe fn offset_correction(in_image: Pin<&mut SLImage>, offset_map: Pin<&mut SLImage>, dark_offset: i32) -> i32;
        unsafe fn gain_correction(in_image: Pin<&mut SLImage>, gain_map: Pin<&mut SLImage>, dark_offset: i32) -> i32;
        unsafe fn kernel_defect_correction(in_image: Pin<&mut SLImage>, defect_map: Pin<&mut SLImage>) -> i32;
        fn get_data_pointer(image: Pin<&mut SLImage>, frame: i32) -> &mut [u16];
    }
}

pub struct SLDevice {
    inner: UniquePtr<ffi::SLDevice>,
}

unsafe impl Send for SLDevice {}
unsafe impl Sync for SLDevice {}

impl SLDevice {
    pub fn new(interface: DeviceInterface) -> Self {
        Self {
            inner: ffi::new_sl_device(interface),
        }
    }

    pub fn is_connected(&mut self) -> bool {
        ffi::is_connected(self.inner.pin_mut())
    }

    pub fn open_camera(&mut self) -> Result<(), SLError> {
        get_error(ffi::open_camera(self.inner.pin_mut()))
    }

    pub fn close_camera(&mut self) -> Result<(), SLError> {
        get_error(ffi::close_camera(self.inner.pin_mut()))
    }
    
    pub fn start_stream(&mut self, exposure_time: Option<Duration>) -> Result<(), SLError> {
        get_error(
            match exposure_time {
                Some(exp_time) => ffi::start_stream_exp_time(self.inner.pin_mut(), exp_time.as_millis() as i32),
                None => ffi::start_stream(self.inner.pin_mut()),
        })
    }
    
    pub fn stop_stream(&mut self) -> Result<(), SLError> {
        get_error(ffi::stop_stream(self.inner.pin_mut()))
    }

    pub fn acquire_image(&mut self, buffer: &mut [u16]) -> SLBufferInfo {
        ffi::acquire_image(self.inner.pin_mut(), buffer)
    }

    pub fn software_trigger(&mut self) -> Result<(), SLError> {
        get_error(ffi::software_trigger(self.inner.pin_mut()))
    }

    pub fn set_exposure_time(&mut self, exposure_time: Duration) -> Result<(), SLError> {
        get_error(ffi::set_exposure_time(self.inner.pin_mut(), exposure_time.as_millis() as i32))
    }

    pub fn set_exposure_mode(&mut self, exposure_mode: ffi::ExposureModes) -> Result<(), SLError> {
        get_error(ffi::set_exposure_mode(self.inner.pin_mut(), exposure_mode))
    }

    pub fn set_number_of_frames(&mut self, frame_count: u32) -> Result<(), SLError> {
        get_error(ffi::set_number_of_frames(self.inner.pin_mut(), frame_count as i32))
    }

    pub fn get_image_x_dim(&mut self) -> Result<u32, SLError> {
        ffi::get_image_x_dim(self.inner.pin_mut())
            .try_into()
            .map_err(|_| SLError::Unknown)
    }

    pub fn get_image_y_dim(&mut self) -> Result<u32, SLError> {
        ffi::get_image_y_dim(self.inner.pin_mut())
            .try_into()
            .map_err(|_| SLError::Unknown)
    }

    // pub fn set_roi(&mut self, roi: &mut ffi::ROIinfo) -> Result<(), SLError> {
    //     get_error(ffi::set_roi(self.inner.pin_mut(), roi))
    // }

    // pub fn get_roi(&mut self) -> Result<ffi::ROIinfo, SLError> {
    //     let mut roi = ffi::ROIinfo { X: 0, Y: 0, W: 0, H: 0};
    //     match get_error(ffi::set_roi(self.inner.pin_mut(), &mut roi)) {
    //         Ok(_) => Ok(roi),
    //         Err(e) => Err(e),
    //     }    
    // }

    /*
    pub fn get_model_info(&mut self) -> ModelInfo {
        ffi::get_model_info(self.inner.pin_mut())
    }
    */
}

pub struct SLImage {
    inner: UniquePtr<ffi::SLImage>
}

unsafe impl Send for SLImage {}
unsafe impl Sync for SLImage {}

impl SLImage {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            inner: ffi::new_sl_image(width as i32, height as i32),
        }
    }

    pub fn read_tiff_file(filename: &CxxString) -> Result<Self, ()> {
        let mut image: SLImage = SLImage {
            inner: ffi::new_sl_image(1, 1)
        };

        if ffi::read_tiff_image(filename, image.inner.pin_mut()) {
            Ok(image)
        }
        else {
            Err(())
        }
    }

    pub fn get_data_pointer(&mut self, frame: u32) -> *mut u16 {
        ffi::get_data_pointer(self.inner.pin_mut(), frame as i32)
    }

    pub fn apply_offset_correction(&mut self, dark_map: &mut SLImage, dark_offset: u32) -> Result<(), SLError> {
        unsafe { get_error(ffi::offset_correction(self.inner.pin_mut(), dark_map.inner.pin_mut(), dark_offset as i32)) }
    }

    pub fn apply_gain_correction(&mut self, gain_map: &mut SLImage, dark_offset: u32) -> Result<(), SLError> {
        unsafe { get_error(ffi::gain_correction(self.inner.pin_mut(), gain_map.inner.pin_mut(), dark_offset as i32)) }
    }

    pub fn apply_kernel_defect_correction(&mut self, defect_map: &mut SLImage) -> Result<(), SLError> {
        unsafe { get_error(ffi::kernel_defect_correction(self.inner.pin_mut(), defect_map.inner.pin_mut())) }
    }
}

#[cfg(test)]
mod tests {
}
