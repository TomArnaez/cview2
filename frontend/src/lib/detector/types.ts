import type { CaptureReport, DetectorSpecification, DetectorStatus, Duration } from "../../bindings";

export type Detector = {
    id: string,
    status: DetectorStatus,
    specification: DetectorSpecification,
    defectMap: boolean,
    darkMapExposures: Duration[],
    captureReport: CaptureReport | null
};