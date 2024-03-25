export type DetectorStatus = "Idle" | "Disconnected" | "Capturing";

export type DetectorSpecification = {
    width: number,
    height: number
}

export type Detector = {
    id: string,
    specification: DetectorSpecification,
    status: DetectorStatus
};

export type CaptureMode = ({type: "SignalAccumulationCapture"} & SignalAccumulationCapture)

export type SignalAccumulationCapture = {
    expTimes: number[],
    frames: number,
}