#include "C:\\dev\\repos\\cview2\\wrapper\\include\\test.h"
#include <iostream>

std::unique_ptr<SLDevice> new_sl_device() {
  return std::unique_ptr<SLDevice>(new SLDevice(USB));
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

int start_stream(SLDevice& device, int exp_time_ms) {
  return int(device.StartStream(exp_time_ms));
}

int go_unlive(SLDevice& device) {
  return int(device.GoUnLive());
}

bool read_frame(SLDevice& device, unsigned short* data_ptr) {
  return device.ReadFrame(data_ptr);
}

int set_exposure_time(SLDevice& device, int exp_time_ms) {
  return int(device.SetExposureTime(exp_time_ms));
}

int get_image_x_dim(SLDevice& device) {
    return device.GetImageXDim();
}

int get_image_y_dim(SLDevice& device) {
    return device.GetImageYDim();
}

std::unique_ptr<SLImage> new_sl_image(int width, int height) {
  return std::unique_ptr<SLImage>(new SLImage(width, height));
}

ushort* get_data_pointer(SLImage& image, int frame) {
  return image.GetDataPointer(frame);
}

bool read_tiff_image(std::string filename, SLImage& out_image) {
  return SLImage::ReadTiffImage(filename, out_image);
}

bool write_tiff_image(std::string filename, SLImage& in_image, int bits) {
  return SLImage::WriteTiffImage(filename, in_image, 16);
}

SLError offset_correction(SLImage& in_image, SLImage& offset_image, int dark_offset) {
  return SLImage::OffsetCorrection(in_image, &offset_image, dark_offset);
}

SLError gain_correction(SLImage& in_image, SLImage& gain_image, int dark_offset) {
  return SLImage::GainCorrection(in_image, &gain_image, dark_offset);
}

SLError kernel_defect_correction(SLImage& in_image, SLImage& defect_map) {
  return SLImage::KernelDefectCorrection(in_image, in_image, &defect_map);
}