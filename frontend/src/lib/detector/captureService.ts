import { writable } from "svelte/store";
import type { CaptureReport } from "../../bindings";

export async function createCaptureService() {
    const { subscribe, set } = writable<CaptureReport[]>([]);
}