import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import type { ImageDetails } from "../../bindings";

export function CreateImageStore() {
    const store = writable<ImageDetails[]>([]);
    const { subscribe, set } = store;

    const unlisten = listen<ImageDetails[]>("image-manager-state-changed", (event) => {
        set(event.payload);
    });

    return { subscribe };
}

export const imageStore = CreateImageStore();