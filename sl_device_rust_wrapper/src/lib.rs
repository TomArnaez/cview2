use cxx::UniquePtr;

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

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("C:\\dev\\repos\\SLDeviceRustWrapper\\sl_device_rust_wrapper\\include\\test.h");
        include!("SLDevice.h");
        type SLDevice;

        fn new_sl_device() -> UniquePtr<SLDevice>;

        fn open_camera(device: Pin<&mut SLDevice>) -> i32;
        fn close_camera(device: Pin<&mut SLDevice>) -> i32;
        fn start_stream(device: Pin<&mut SLDevice>, exp_time_ms: u32) -> i32;
        fn go_unlive(device: Pin<&mut SLDevice>) -> i32;

        //fn set_exposure_time(device: Pin<&mut SLDevice>, exp_time_ms: u32) -> i32;
        
        fn get_image_x_dim(device: Pin<&mut SLDevice>) -> i32;
        fn get_image_y_dim(device: Pin<&mut SLDevice>) -> i32;
    }
}

struct SLDevice {
    inner: UniquePtr<ffi::SLDevice>,
}

impl SLDevice {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_sl_device(),
        }
    }

    pub fn open_camera(&mut self) -> Result<(), SLError> {
        let err = SLError::from_i32(ffi::open_camera(self.inner.pin_mut())).unwrap();
        match err {
            SLError::Success => Ok(()),
            _ => Err(err)
        }
    }

    pub fn close_camera(&mut self) -> Result<(), SLError> {
        let err = SLError::from_i32(ffi::close_camera(self.inner.pin_mut())).unwrap();
        match err {
            SLError::Success => Ok(()),
            _ => Err(err)
        }
    }

    pub fn start_stream(&mut self, exp_time_ms: u32) -> Result<(), SLError> {
        let err = SLError::from_i32(ffi::start_stream(self.inner.pin_mut(), exp_time_ms)).unwrap();
        match err {
            SLError::Success => Ok(()),
            _ => Err(err)
        }
    }

    pub fn go_unlive(&mut self, exp_time_ms: u32) -> Result<(), SLError> {
        let err = SLError::from_i32(ffi::go_unlive(self.inner.pin_mut())).unwrap();
        match err {
            SLError::Success => Ok(()),
            _ => Err(err)
        }
    }
    
    /*
    pub fn set_exposure_time(&mut self, exp_time_ms: u32) -> Result<(), SLError> {
        let err = SLError::from_i32(ffi::set_exposure_time(self.inner.pin_mut(), exp_time_ms)).unwrap();
        match err {
            SLError::Success => Ok(()),
            _ => Err(err)
        }
    }
    */

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut device = SLDevice::new();
    }
}
