import { writable } from "svelte/store";
import { invoke, listen } from "../backend/ipc";
import type { Detector } from "./types";

export const detectorStore = writable<Detector[]>([]);

function subscribeToDetectors(callback: (detectors: Detector[]) => void) {
    return listen<any>('detectors', (event) => {
        callback(event.payload)
    })
}