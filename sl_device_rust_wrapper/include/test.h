// include/blobstore.h

#pragma once
#include <memory>
#include "SLDevice.h"

using namespace SpectrumLogic;

std::unique_ptr<SLDevice> new_sl_device();

int open_camera(SLDevice& device);
int close_camera(SLDevice& device);
int start_stream(SLDevice& device, int exp_time_ms);
int go_unlive(SLDevice& device);

int set_exposure_time(SLDevice& device, int exp_time_ms);

int get_image_x_dim(SLDevice& device);
int get_image_y_dim(SLDevice& device);
