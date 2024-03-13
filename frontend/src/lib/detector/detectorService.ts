import { writable } from "svelte/store";
import type { Detector } from "./types";

export function createDetectorService() {
    const { subscribe, set } = writable<Detector[]>([]);

    return { subscribe }
}

export type DetectorService = ReturnType<typeof createDetectorService>;