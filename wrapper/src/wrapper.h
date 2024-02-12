#pragma once
#include <memory>
#include "rust/cxx.h"
#include <iostream>
#include "wrapper/src/lib.rs.h"

using namespace SpectrumLogic;

SLDeviceInfoRS convertDeviceInfo(const SLDeviceInfo& deviceInfo) {
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

namespace SLBindings {
    template<typename T, typename... Args>
    std::unique_ptr<T>
    construct(Args... args) {
        return std::unique_ptr<T>(new T(args...));
    }

    SLDeviceInfoRS get_device_info(SLDevice& device) {
        return convertDeviceInfo(device.GetDeviceInfo());
    }

    rust::Vec<SLDeviceInfoRS> scan_cameras() {
        auto devices = SLDevice::ScanCameras();
        rust::Vec<SLDeviceInfoRS> devicesRs;

        for (const auto& device : devices) 
            devicesRs.push_back(convertDeviceInfo(device));

        return devicesRs;
    }

    SLError kernel_defect_correction(SLImage& image, const SLImage& defect_map) {
        return SLImage::KernelDefectCorrection(image, image, const_cast<SLImage*>(&defect_map));
    }
}
