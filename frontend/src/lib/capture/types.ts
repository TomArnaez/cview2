export type CaptureSettings = {
    dds: boolean,
    roi: ROI,
    fullWellMode: FullWellModes,
    testMode: boolean,
    timeout: number,
}

export type ROI = {
    x: number
    y: number
    width: number
    height: number
};

export enum FullWellModes {
    Low,
    High,
    Unknown
}

export type CaptureReport = {
    id: String,
    name: String,
    taskCount: String,
    completedTaskCount: String,
};