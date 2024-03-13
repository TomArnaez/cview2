import { writable } from "svelte/store";
import type { Image } from "./types";
import { invoke } from "../backend/ipc";

export async function createImageController() {
    const { subscribe, set } = writable<Image[]>([])

    //set(await invoke<Image[]>("list_all_images"));

    function saveImageAsBitmap(image: Image) {
        invoke<any>("save_image_as_bitmap");
    }

    function saveImageAsTiff(image: Image) {
        invoke<any>("save_image_as_tiff");
    }

    return  {
        subscribe,
        saveImageAsBitmap,
        saveImageAsTiff
    }
}

export type ImageController = ReturnType<typeof createImageController>;