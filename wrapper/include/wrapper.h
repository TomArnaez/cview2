#pragma once
#include <memory>
#include "SLDevice.h"
#include "SLImage.h"

using namespace SpectrumLogic;

std::unique_ptr<SLDevice> new_sl_device(SpectrumLogic::DeviceInterface device_interface);
bool is_connected(SLDevice& device);
int open_camera(SLDevice& device);
int close_camera(SLDevice& device);
int start_stream(SLDevice& device, int exp_time_ms);
int go_live(SLDevice& device);
int go_unlive(SLDevice& device);
int software_trigger(SLDevice& device);
bool read_frame(SLDevice& device, unsigned short* data_ptr);
int set_exposure_time(SLDevice& device, int exp_time_ms);
int set_exposure_mode(SLDevice& device, ExposureModes exposure_mdoe);
int set_number_of_frames(SLDevice& device, int exp_time_ms);
int get_image_x_dim(SLDevice& device);
int get_image_y_dim(SLDevice& device);
bool read_frame(SLDevice& device, unsigned short* data_ptr);
int read_buffer(SLDevice& device, unsigned short* data_ptr, int buf_num, int timeout);
int set_roi(SLDevice& device, SpectrumLogic::ROIinfo& roi);
int get_roi(SLDevice& device, SpectrumLogic::ROIinfo& out_roi);
// ModelInfo get_model_info(SLDevice& device);

std::unique_ptr<SLImage> new_sl_image(int width, int height);
bool read_tiff_image(const std::string& filename, SLImage& image);
bool write_tiff_image(const std::string& filename, SLImage& in_image, int bits);
int offset_correction(SLImage& in_image, SLImage& offset_image, int dark_offset);
int gain_correction(SLImage& in_image, SLImage& gain_image, int dark_offset);
int kernel_defect_correction(SLImage& in_image, SLImage& defect_map);
ushort* get_data_pointer(SLImage& image, int frame);