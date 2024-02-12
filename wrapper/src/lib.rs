use std::time::Duration;
use cxx::{type_id, Exception, ExternType, UniquePtr};
use serde::{Deserialize, Serialize};
pub use sldevice_ffi::{DeviceInterface, ExposureModes, FullWellModes, ROIinfo, SLDeviceInfo, SLError, scan_cameras};

const ACQUISITION_TIMEOUT_DEFAULT: u32 = 1000;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(C)] 
pub struct SLBufferInfo { 
    pub error: SLError, 
    pub width: u32,
    pub height: u32,
    pub size: u32, 
    pub missing_packets: u32, 
    pub frame_count: u32, 
    pub block_id: u64, 
    pub timestamp: u64, 
}

unsafe impl ExternType for SLBufferInfo {
    type Id = type_id!("SpectrumLogic::SLBufferInfo");
    type Kind = cxx::kind::Trivial;
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct ROI {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

unsafe impl ExternType for ROI {
    type Id = type_id!("SpectrumLogic::ROIinfo");
    type Kind = cxx::kind::Trivial;
}

pub struct RegisterAddress(u32);

#[cxx::bridge(namespace = "SpectrumLogic")]
mod sldevice_ffi {
    #[derive(Debug, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum DeviceInterface {
		CL = 0,
		USB = 1,
		PLEORA = 3,
		S2I_GIGE = 4,
		EIO_USB = 5,
		UNKNOWN = 6
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum ExposureModes {
        #[rust_name="Unknown"]
        unknown = 0,
        #[rust_name="SequenceMode"]
        seq_mode,
        #[rust_name="FPS25Mode"]
        fps25_mode,
        #[rust_name="FPS30Mode"]
        fps30_mode,
        #[rust_name="TriggerMode"]
        trig_mode,
        #[rust_name="XFPSMode"]
        xfps_mode,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum SLError {
        SL_ERROR_SUCCESS = 0,
        SL_ERROR_INVALID_PARAM,
        SL_ERROR_NO_DEVICE,
        SL_ERROR_NOT_FOUND,
        SL_ERROR_BUSY,
        SL_ERROR_TIMEOUT,
        SL_ERROR_CORRECTION,
        SL_ERROR_NOT_SUPPORTED,
        SL_ERROR_ALREADY_EXISTS,
        SL_ERROR_INTERNAL,
        SL_ERROR_OTHER,
        SL_ERROR_DEVICE_CLOSED,
        SL_ERROR_DEVICE_STREAMING,
        SL_ERROR_CONFIG_FAILED,
        SL_ERROR_CONFIG_FILE_NOT_FOUND,
        SL_ERROR_NOT_ENOUGH_MEMORY,
        SL_ERROR_OVERFLOW,
        SL_ERROR_PIPE,
        SL_ERROR_INTERRUPTED,
        SL_ERROR_IO,
        SL_ERROR_ACCESS,
        SL_ERROR_REQUIRES_ADMIN,
        SL_ERROR_CRITICAL,
        SL_ERROR_NOT_INIT,
        SL_ERROR_NOT_FILLED,
        SL_ERROR_ABORTED,
        SL_ERROR_RESENDS,
        SL_ERROR_MISSING_PACKETS,
        SL_ERROR_READ_FAILED,
        SL_ERROR_WRITE_FAILED,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum FullWellModes {
        Low = 0,
        High = 2,
        Unknown = 3
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[cxx_name="SLDeviceInfoRS"]
    pub struct SLDeviceInfo {
        pub device_interface: DeviceInterface,
        pub detector_ip_address: String,
        pub id: String,
        pub unit: u32,
        params: String,
        force_ip: String,
        log_file_path: String,
    }

    unsafe extern "C++" {
        include!("SLDevice.h");
        include!("wrapper/src/wrapper.h");

        type SLDevice;
        type ExposureModes;
        type DeviceInterface;
        type FullWellModes;
        type SLError;
        type SLBufferInfo = crate::SLBufferInfo;
        type ROIinfo = crate::ROI;

        #[rust_name="constuct_sldevice_with_interface"]
        #[namespace="SLBindings"]
        pub fn construct(device_interface: DeviceInterface) -> Result<UniquePtr<SLDevice>>;
        // #[rust_name="constuct_sldevice_with_devinfo"]
        // #[namespace="SLBindings"]
        // pub fn construct(device_info: SLDeviceInfo) -> UniquePtr<SLDevice>;
        unsafe fn AcquireImage(self: Pin<&mut SLDevice>, buffer: *mut u16, timeout_ms: u32) -> SLBufferInfo;
        fn SetExposureTime(self: Pin<&mut SLDevice>, exposure_time_ms: i32) -> SLError;
        fn SetExposureMode(self: Pin<&mut SLDevice>, exposure_mode: ExposureModes) -> SLError;
        fn IsConnected(self: Pin<&mut SLDevice>) -> bool;
        fn OpenCamera(self: Pin<&mut SLDevice>, bufferDepth: i32) -> SLError;
        fn CloseCamera(self: Pin<&mut SLDevice>) -> SLError;
        fn StartStream(self: Pin<&mut SLDevice>) -> SLError;
        fn StopStream(self: Pin<&mut SLDevice>) -> SLError;
        fn GetImageXDim(self: Pin<&mut SLDevice>) -> i32;
        fn GetImageYDim(self: Pin<&mut SLDevice>) -> i32;
        fn GetROI(self: Pin<&mut SLDevice>, roi: &mut ROIinfo) -> SLError;
        fn SetROI(self: Pin<&mut SLDevice>, roi: ROIinfo) -> SLError;
        fn RegisterWrite(self: Pin<&mut SLDevice>, addr: i32, value: i32, sensor_num: i32) -> SLError;
        fn RegisterRead(self: Pin<&mut SLDevice>, addr: i32, sensor_num: i32) -> i32;
        fn SetNumberOfFrames(self: Pin<&mut SLDevice>, num_frames: i32) -> SLError;
        fn SoftwareTrigger(self: Pin<&mut SLDevice>) -> SLError;
        fn MeasureTemperature(self: Pin<&mut SLDevice>, temp_out: &mut f32, sensor_num: i32) -> SLError;
        fn SetTestMode(self: Pin<&mut SLDevice>, test_mode_on: bool) -> SLError;
        fn SetDDS(self: Pin<&mut SLDevice>, dds_on: bool) -> SLError;
        fn SetFullWell(self: Pin<&mut SLDevice>, full_well_mode: FullWellModes) -> SLError;
        #[namespace="SLBindings"]
        fn get_device_info(device: Pin<&mut SLDevice>) -> SLDeviceInfo;
        #[namespace="SLBindings"]
        fn scan_cameras() -> Result<Vec<SLDeviceInfo>>;
    }
}

#[cxx::bridge(namespace = "SpectrumLogic")]
pub mod slimage_ffi {
    unsafe extern "C++" {
        include!("SLImage.h");
        
        type SLImage;

        #[rust_name="constuct_slimage"]
        #[namespace="SLBindings"]
        fn construct() -> Result<UniquePtr<SLImage>>;
        #[rust_name="constuct_slimage_width_height"]
        #[namespace="SLBindings"]
        fn construct(width: i32, height: i32) -> Result<UniquePtr<SLImage>>;
        #[rust_name="constuct_slimage_width_height_depth"]
        #[namespace="SLBindings"]
        fn construct(width: i32, height: i32, depth: i32) -> Result<UniquePtr<SLImage>>;
        fn GetHeight(self: &SLImage) -> i32;
        fn GetWidth(self: &SLImage) -> i32;
        fn GetDepth(self: &SLImage) -> i32;
        unsafe fn GetDataPointer(self: Pin<&mut SLImage>, frame: i32) -> *mut u16;
       // unsafe fn KernelDefectCorrection(self, in_img: Pin<&mut SLImage>, out_img: Pin<&mut SLImage>, defect_map: *mut SLImage) -> SLError;
    }
}

fn slerror_to_result(error_code: SLError) -> Result<(), SLError> {
    match error_code {
        SLError::SL_ERROR_SUCCESS => Ok(()),
        _ => Err(error_code), 
    }
}

pub struct SLDevice {
    device: UniquePtr<sldevice_ffi::SLDevice>,
}

impl SLDevice {
    pub fn new(interface: DeviceInterface) -> Result<Self, String> {
        match sldevice_ffi::constuct_sldevice_with_interface(interface) {
            Ok(device) => {
                Ok(Self {
                    device
                })
            },
            Err(exception) => {
                Err(exception.what().to_string())
            }
        }
    }

    // pub fn new(device_info: SLDeviceInfo) -> Self {
    //     Self {
    //         device: sldevice_ffi::construct_sldevice_with_devinfo(device_info)
    //     }
    // }

    pub fn scan_cameras() -> Result<Vec<SLDeviceInfo>, Exception> {
        sldevice_ffi::scan_cameras()
    }

    pub fn register_read(&mut self, address: RegisterAddress, sensor_sum: u32) -> i32 {
        self.device.pin_mut().RegisterRead(address.0 as i32, sensor_sum as i32)
    }

    pub fn open_camera(&mut self) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().OpenCamera(100))
    }

    pub fn close_camera(&mut self) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().CloseCamera())
    }

    pub fn is_connected(&mut self) -> bool {
        self.device.pin_mut().IsConnected()
    }

    pub fn start_stream(&mut self) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().StartStream())
    }

    pub fn stop_stream(&mut self) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().StopStream())
    }

    pub fn software_trigger(&mut self) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SoftwareTrigger())
    }

    pub fn set_number_of_frames(&mut self, frames: u32) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SetNumberOfFrames(frames as i32))
    }

    pub fn set_full_well_mode(&mut self, full_well_mode: FullWellModes) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SetFullWell(full_well_mode))
    }

    pub fn get_roi(&mut self) -> Result<ROI, SLError> {
        let mut roi = ROI::default();
        match self.device.pin_mut().GetROI(&mut roi) {
            SLError::SL_ERROR_SUCCESS => Ok(roi),
            e => Err(e)
        }
    }

    pub fn set_roi(&mut self, roi: ROI) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SetROI(roi))
    }

    pub fn measure_temperature(&mut self, sensor: u32) -> Result<f32, SLError> {
        let mut temp = 0.;
        match self.device.pin_mut().MeasureTemperature(&mut temp, sensor as i32) {
            SLError::SL_ERROR_SUCCESS => Ok(temp),
            e => Err(e)
        }
    }

    pub fn get_image_dims(&mut self) -> Result<(u32, u32), SLError> {
        let (x, y) = (self.device.pin_mut().GetImageXDim(), self.device.pin_mut().GetImageYDim());
        if x == -1 || y == -1 {
            Err(SLError::SL_ERROR_INTERNAL)
        } else {
            Ok((x as u32, y as u32))
        }
    }

    pub fn set_exposure_mode(&mut self, exposure_mode: sldevice_ffi::ExposureModes) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SetExposureMode(exposure_mode))
    }

    pub fn set_exposure_time(&mut self, exposure_time: Duration) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SetExposureTime(exposure_time.as_millis() as i32))
    }

    pub fn set_dds(&mut self, dds_on: bool) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SetDDS(dds_on))
    }

    pub fn set_test_mode(&mut self, test_mode_on: bool) -> Result<(), SLError> {
        slerror_to_result(self.device.pin_mut().SetTestMode(test_mode_on))
    }

    pub fn acquire_image(&mut self, buffer: &mut [u16], timeout: Option<Duration>) -> Result<SLBufferInfo, SLError> {
        let buffer_info;
        unsafe {
            buffer_info = self.device.pin_mut().AcquireImage(buffer.as_mut_ptr() as *mut u16,
            timeout.map_or(ACQUISITION_TIMEOUT_DEFAULT, |d| d.as_millis() as u32));
        }
        match buffer_info.error {
            SLError::SL_ERROR_SUCCESS | SLError::SL_ERROR_MISSING_PACKETS => Ok(buffer_info),
            e => Err(e.clone())
        }
    }
}

unsafe impl Sync for SLDevice {}
unsafe impl Send for SLDevice {}

pub struct SLImage {
    image: UniquePtr<slimage_ffi::SLImage>
}

impl SLImage {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: slimage_ffi::constuct_slimage_width_height(width as i32, height as i32).unwrap()
        }
    }

    pub fn new_stack(width: u32, height: u32, depth: u32) -> Self {
        Self {
            image: slimage_ffi::constuct_slimage_width_height_depth(width as i32, height as i32, depth as i32).unwrap()
        }
    }

    pub fn width(&self) -> u32 {
        self.image.GetWidth() as u32
    }

    pub fn height(&self) -> u32 {
        self.image.GetHeight() as u32
    }

    pub fn depth(&self) -> u32 {
        self.image.GetDepth() as u32
    }

    pub fn get_frame_data(&mut self, frame: u32) -> &[u16] {
        unsafe {
            std::slice::from_raw_parts(self.image.pin_mut().GetDataPointer(frame as i32), (self.height() * self.width()) as usize)
        }
    }

    pub fn get_frame_data_mut(&mut self, frame: u32) -> &mut [u16] {
        unsafe {
            std::slice::from_raw_parts_mut(self.image.pin_mut().GetDataPointer(frame as i32), (self.height() * self.width()) as usize)
        }
    }
}

unsafe impl Sync for SLImage {}
unsafe impl Send for SLImage {}