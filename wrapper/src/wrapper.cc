#include "C:\\dev\\repos\\cview2\\wrapper\\include\\wrapper.h"
#include <iostream>

// SLDevice

std::unique_ptr<SLDevice> new_sl_device(DeviceInterface device_interface) {
  return std::unique_ptr<SLDevice>(new SLDevice(device_interface));
}

bool is_connected(SLDevice& device) {
  return device.IsConnected();
}

int open_camera(SLDevice& device) {
    return int(device.OpenCamera());
}

int close_camera(SLDevice& device) {
    return int(device.CloseCamera());
}

int start_stream(SLDevice& device) {
  return int(device.StartStream());
}

int start_stream(SLDevice& device, int exp_time_ms) {
  return int(device.StartStream(exp_time_ms));
}

int stop_stream(SLDevice& device) {
  return int(device.StopStream());
}

int software_trigger(SLDevice& device) {
  return int(device.SoftwareTrigger());
}

SLBufferInfo acquire_image(SLDevice& device, rust::Slice<u16> buffer) {
  return int(device.AcquireImage(buffer.data()));
}

int set_exposure_time(SLDevice& device, int exp_time_ms) {
  return int(device.SetExposureTime(exp_time_ms));
}

int set_exposure_mode(SLDevice& device, ExposureModes exposure_mode) {
  return int(device.SetExposureMode(exposure_mode));
}

int set_number_of_frames(SLDevice& device, int num_frames) {
  return int(device.SetNumberOfFrames(num_frames));
}

int get_image_x_dim(SLDevice& device) {
    return device.GetImageXDim();
}

int get_image_y_dim(SLDevice& device) {
    return device.GetImageYDim();
}

int set_roi(SLDevice& device, ROIinfo& roi) {
  return int(device.SetROI(roi));
}
int get_roi(SLDevice& device, ROIinfo& out_roi) {
  return int(device.GetROI(out_roi));
}

/*
ModelInfo get_model_info(SLDevice& device) {
  return device.GetModelInfo();
}
*/

// SLImage

std::unique_ptr<SLImage> new_sl_image(int width, int height) {
  return std::unique_ptr<SLImage>(new SLImage(width, height));
}

ushort* get_data_pointer(SLImage& image, int frame) {
  return image.GetDataPointer(frame);
}

bool read_tiff_image(const std::string& filename, SLImage& out_image) {
  return SLImage::ReadTiffImage(filename, out_image);
}

bool write_tiff_image(const std::string& filename, SLImage& in_image, int bits) {
  return SLImage::WriteTiffImage(filename, in_image, 16);
}

int offset_correction(SLImage& in_image, SLImage& offset_image, int dark_offset) {
  return int(SLImage::OffsetCorrection(in_image, &offset_image, dark_offset));
}

int gain_correction(SLImage& in_image, SLImage& gain_image, int dark_offset) {
  return int(SLImage::GainCorrection(in_image, &gain_image, dark_offset));
}

int kernel_defect_correction(SLImage& in_image, SLImage& defect_map) {
  return int(SLImage::KernelDefectCorrection(in_image, in_image, &defect_map));
}