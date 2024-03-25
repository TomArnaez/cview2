import { writable } from "svelte/store";
import type { CaptureMode, Detector } from "./types";
import { invoke, listen } from "../backend/ipc";

export async function createDetectorService() {
    const { subscribe, set } = writable<Detector[]>([]);

    set(await listDetectors());

    subscribe(dets => {
        console.log(dets)
    })

    async function runCapture(detectorId: string, capture: CaptureMode) {
        try {
            await invoke<any>("run_capture", {detector_id: detectorId, capture});
        } catch (e: any) {
            console.log(e);
        }
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

async function listDetectors(): Promise<Detector[]> {
    return await invoke<Detector[]>("list_all_detectors");
}

function subscribeToDetectors(callback: (detectors: Detector[]) => void) {
    return listen<any>('detectors', (event) => {
        callback(event.payload)
    })
}

export const detectorService = await createDetectorService();
export type DetectorService = ReturnType<typeof createDetectorService>;