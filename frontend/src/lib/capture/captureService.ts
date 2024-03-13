import { readable } from "svelte/store";

export async function createCaptureService() {
    
    
    function cancelCapture() {
    }

    return {
        cancelCapture
    }
}

export type CaptureService = ReturnType<typeof createCaptureService>;