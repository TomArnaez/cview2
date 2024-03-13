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