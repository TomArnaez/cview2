export type Detector = {
    id: string,
    specification: DetectorSpecification,
    status: DetectorStatus,
    defectMap: boolean,
    darkMapExposures: number[]
};

export type DetectorSpecification = {
    width: number,
    height: number
}

export type DetectorStatus = "Idle" | "Disconnected" | "Capturing";

export type CaptureMode = 
    | ({ type: "StreamCapture" } & StreamCapture) 
    | ({ type: "SignalAccumulationCapture" } & SignalAccumulationCapture) 

export type SignalAccumulationCapture = {
    expTimes: number[],
    frames: number,
}

export type StreamCapture = {
    expTime: number,
}