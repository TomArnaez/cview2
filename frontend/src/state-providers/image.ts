import { writable } from "svelte/store";

export function createImageState() {
    const state = writable({
    })
}

export type ImageState = ReturnType<typeof createImageState>;