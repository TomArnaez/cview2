use cxx::{UniquePtr, CxxString};

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
mod ffi {
    unsafe extern "C++" {
        include!("C:\\dev\\repos\\cview2\\wrapper\\include\\test.h");

        include!("SLDevice.h");
        type SLDevice;
        
        fn new_sl_device() -> UniquePtr<SLDevice>;
        fn open_camera(device: Pin<&mut SLDevice>) -> i32;
        fn close_camera(device: Pin<&mut SLDevice>) -> i32;
        fn start_stream(device: Pin<&mut SLDevice>, exp_time_ms: i32) -> i32;
        fn go_unlive(device: Pin<&mut SLDevice>) -> i32;
        fn is_connected(device: Pin<&mut SLDevice>) -> bool;
        unsafe fn read_frame(device: Pin<&mut SLDevice>, data_ptr: *mut u16) -> bool;
        //fn set_exposure_time(device: Pin<&mut SLDevice>, exp_time_ms: u32) -> i32;
        fn get_image_x_dim(device: Pin<&mut SLDevice>) -> i32;
        fn get_image_y_dim(device: Pin<&mut SLDevice>) -> i32;

        include!("SLImage.h");
        type SLImage;
        
        fn new_sl_image(width: i32, height: i32) -> UniquePtr<SLImage>;
        fn read_tiff_image(filename: &CxxString, image: Pin<&mut SLImage>) -> bool;
        unsafe fn offset_correction(in_image: Pin<&mut SLImage>, offset_map: Pin<&mut SLImage>, dark_offset: u32) -> i32;
        unsafe fn gain_correction(in_image: Pin<&mut SLImage>, gain_map: Pin<&mut SLImage>, dark_offset: u32) -> i32;
        unsafe fn kernel_defect_correction(in_image: Pin<&mut SLImage>, defect_map: Pin<&mut SLImage>) -> i32;
        fn get_data_pointer(image: Pin<&mut SLImage>, frame: i32) -> *mut u16;
    }
}

pub struct SLDevice {
    inner: UniquePtr<ffi::SLDevice>,
}

unsafe impl Send for SLDevice {}
unsafe impl Sync for SLDevice {}

impl SLDevice {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_sl_device(),
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

    pub fn start_stream(&mut self, exp_time_ms: u32) -> Result<(), SLError> {
        get_error(ffi::start_stream(self.inner.pin_mut(), exp_time_ms as i32))
    }

    pub fn go_unlive(&mut self, exp_time_ms: u32) -> Result<(), SLError> {
        get_error(ffi::go_unlive(self.inner.pin_mut()))
    }

    pub fn read_frame(&mut self, data_ptr: *mut u16) -> bool {
        unsafe { ffi::read_frame(self.inner.pin_mut(), data_ptr) }
    }

    // pub fn set_exposure_time(&mut self, exp_time_ms: u32) -> Result<(), SLError> {
    //     let err = SLError::from_i32(ffi::set_exposure_time(self.inner.pin_mut(), exp_time_ms)).unwrap();
    //     match err {
    //         SLError::Success => Ok(()),
    //         _ => Err(err)
    //     }
    // }

    pub fn get_image_x_dim(&mut self) -> Result<u32, SLError> {
        ffi::get_image_x_dim(self.inner.pin_mut())
            .try_into()
            .map_err(|e| SLError::Unknown)
    }

    pub fn get_image_y_dim(&mut self) -> Result<u32, SLError> {
        ffi::get_image_y_dim(self.inner.pin_mut())
            .try_into()
            .map_err(|e| SLError::Unknown)
    }
}

pub struct SLImage {
    inner: UniquePtr<ffi::SLImage>
}

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
        unsafe { get_error(ffi::offset_correction(self.inner.pin_mut(), dark_map.inner.pin_mut(), dark_offset)) }
    }

    pub fn apply_gain_correction(&mut self, gain_map: &mut SLImage, dark_offset: u32) -> Result<(), SLError> {
        unsafe { get_error(ffi::gain_correction(self.inner.pin_mut(), gain_map.inner.pin_mut(), dark_offset)) }
    }

    pub fn apply_kernel_defect_correction(&mut self, defect_map: &mut SLImage) -> Result<(), SLError> {
        unsafe { get_error(ffi::kernel_defect_correction(self.inner.pin_mut(), defect_map.inner.pin_mut())) }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn it_works() {
        let mut device = SLDevice::new();
        let err = device.open_camera();
        println!("{:?}", err);
    }
}
