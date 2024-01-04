// src/blobstore.cc

#include "C:\\dev\\repos\\SLDeviceRustWrapper\\sl_device_rust_wrapper\\include\\test.h"
#include <iostream>

std::unique_ptr<SLDevice> new_sl_device() {
  return std::unique_ptr<SLDevice>(new SLDevice(USB));
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

int set_exposure_time(SLDevice& device, int exp_time_ms) {
  return int(device.SetExposureTime(exp_time_ms));
}

int get_image_x_dim(SLDevice& device) {
    return device.GetImageXDim();
}

int get_image_x_dim(SLDevice& device) {
    return device.GetImageXDim();
}
