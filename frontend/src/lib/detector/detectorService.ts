import { writable } from "svelte/store";
import type { Detector } from "./types";
import { invoke, listen } from "../backend/ipc";
import type { CaptureSettings, SequenceCapture } from "../../bindings";

export async function createDetectorService() {
    const { subscribe, set } = writable<Detector[]>([]);

    let values = await listAllDetectors();
    if (values != null && values?.length > 0) {
        await runCapture(values[0].id)
    }

    async function runCapture(detectorId: string) {
        let captureSettings: CaptureSettings = {
            dds: false,
            fullWellMode: "Low",
            roi: { x: 0, y: 0, w: 300, h: 300},
            testMode: false,
        }
        let capture: SequenceCapture = {
            captureSettings,
            corrected: false,
            exposureTime: 100,
            frameCount: 10
        }
        await invoke<any>("run_capture", { id: detectorId, capture });
    }

    async function cancelCapture(detectorId: string) {
        try {
            await invoke<any>("cancel_capture", {detectorId} );
        } catch (e: any) {
            console.log(e)
        }
    }

    async function scanCameras() {
        try {
            await invoke<any>("scan_cameras");
        } catch (e: any) {

        }
    }

    return { subscribe }
}

async function listAllDetectors(): Promise<Detector[] | null> {
    return invoke<Detector[]>("list_all_detectors");
}

function subscribeToDetectors(callback: (detectors: Detector[]) => void) {
    return listen<any>('detectors', (event) => {
        callback(event.payload)
    })
}

export const detectorService = await createDetectorService();
export type DetectorService = ReturnType<typeof createDetectorService>;