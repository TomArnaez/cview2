#pragma once
#include <memory>
#include "rust/cxx.h"
#include <iostream>
#include "wrapper/src/lib.rs.h"

using namespace SpectrumLogic;

SLDeviceInfoRS convertDeviceInfoToRust(const SLDeviceInfo& deviceInfo) {
    return SLDeviceInfoRS {
        deviceInfo.Interface,
        deviceInfo.DetectorIPAddress,
        deviceInfo.ID,
        static_cast<uint32_t>(deviceInfo.unit),
        deviceInfo.params,
        deviceInfo.forceIP,
        deviceInfo.logFilePath
    };
}

SLDeviceInfo convertDeviceInfoToCxx(const SLDeviceInfoRS& deviceInfo) {
    return SLDeviceInfo {
        deviceInfo.device_interface,
        std::string(deviceInfo.detector_ip_address),
        std::string(deviceInfo.id),
        static_cast<int>(deviceInfo.unit),
        std::string(deviceInfo.params),
        std::string(deviceInfo.force_ip),
        std::string(deviceInfo.log_file_path)
    };
}

namespace SLBindings {
    template<typename T, typename... Args>
    std::unique_ptr<T>
    construct(Args... args) {
        return std::unique_ptr<T>(new T(args...));
    }

    std::unique_ptr<SLDevice> construct_sldevice_from_devinfo(SLDeviceInfoRS devInfo) {
        return std::unique_ptr<SLDevice>(new SLDevice(convertDeviceInfoToCxx(devInfo)));
    }

    SLDeviceInfoRS get_device_info(SLDevice& device) {
        return convertDeviceInfoToRust(device.GetDeviceInfo());
    }

    rust::Vec<SLDeviceInfoRS> scan_cameras() {
        auto devices = SLDevice::ScanCameras();
        rust::Vec<SLDeviceInfoRS> devicesRs;

        for (const auto& device : devices) 
            devicesRs.push_back(convertDeviceInfoToRust(device));

        return devicesRs;
    }

    SLError kernel_defect_correction(SLImage& image, const SLImage& defect_map) {
        return SLImage::KernelDefectCorrection(image, image, const_cast<SLImage*>(&defect_map));
    }
}
