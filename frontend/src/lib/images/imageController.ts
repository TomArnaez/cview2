import { writable } from "svelte/store";
import type { Image } from "./types";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

export function createImageController() {
    const { subscribe, set } = writable<Image[]>([])

    async function promptForTiffPath(): Promise<string | undefined> {
        return open({ directory: false, filters: [{name: "TIFF Image", extensions: ["tif", "tiff"]}], recursive: true }).then(
			(selectedPath) => {
				if (selectedPath === null) return;
				if (Array.isArray(selectedPath) && selectedPath.length !== 1) return;
				return Array.isArray(selectedPath) ? selectedPath[0] : selectedPath;
			}
		);
    }

    async function openImage() {
        const path = await promptForTiffPath();
        if (!path) return;
        await invoke<Image>("open_image", path);
        let images = await invoke<Image[]>("list_all_images");
        console.log(images);
        set(images)
    }

    function saveImageAsBitmap(image: Image) {
        invoke<any>("save_image_as_bitmap");
    }

    function saveImageAsTiff(image: Image) {
        invoke<any>("save_image_as_tiff");
    }

    return  {
        subscribe,
        openImage,
        saveImageAsBitmap,
        saveImageAsTiff
    }
}

export type ImageController = ReturnType<typeof createImageController>;