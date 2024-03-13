import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import type { Image } from "./types";

export function CreateImageStore() {
    const store = writable<Image[]>([]);
    const { subscribe, set } = store;

    const unlisten = listen<Image[]>("image-manager-state-changed", (event) => {
        set(event.payload);
    });

    return { subscribe };
}

export const imageStore = CreateImageStore();