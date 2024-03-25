import { derived, get, writable } from "svelte/store";
import type { View, ImageViewSettings, ViewWithBuffer, Rect } from "./types";
import { invoke } from "../backend/ipc";
import { open } from "@tauri-apps/plugin-dialog";

function createViewStore(view: ViewWithBuffer) {
    const { update, subscribe } = writable<ViewWithBuffer>(view);

    function setROI(roi: Rect) {
        update(view => ({ ...view, roi }));
    }

    return {
        setROI,
        subscribe
    }
}

export function createViewController() {
    const viewsStore = writable<Array<ReturnType<typeof createViewStore>>>([]);

    const views = derived(viewsStore, $viewsStore => {
        return $viewsStore.map(store => get(store));
    }, []);

    function saveAsTiff(path: String, view: View) {
        invoke<any>("save_image_tiff", { path, id: view.id });
    }

    async function promptForTiffPath(): Promise<string | undefined> {
        return open({ directory: false, filters: [{name: "TIFF Image", extensions: ["tif", "tiff"]}], recursive: true }).then(
			(selectedPath) => {
				if (selectedPath === null) return;
				if (Array.isArray(selectedPath) && selectedPath.length !== 1) return;
				return Array.isArray(selectedPath) ? selectedPath[0] : selectedPath;
			}
		);
    }

    async function setViewSlice(view: View, slice: number) {
        try {
            await invoke("set_view_slice", { id: view.id, slice });
        } catch (err: any) {
            console.log(err)
        }
    }

    async function openImage() {
        let view: View | undefined;
        const path = await promptForTiffPath();

        let resolveIdPromise: (value: View) => void = () => {}; 
        const idPromise = new Promise<View>((resolve) => {
            resolveIdPromise = resolve;
        });

        let settings: ImageViewSettings = {
            histogramEquilisation: true,
            invertColours: true,
            saturationPixelSettings: undefined
        };
    
        const sharedBufferReceivedListener = async (e: any) => {
            let buffer = new Uint8ClampedArray(e.getBuffer());
            const view = await idPromise;
            console.log("Buffer received for view", view);
            window.chrome.webview.removeEventListener("sharedbufferreceived", sharedBufferReceivedListener);
            let viewStore = createViewStore({...view, buffer, roi: null });
            viewsStore.update(curr => {
                return [... curr, viewStore];
            })
        };
        
        window.chrome.webview.addEventListener("sharedbufferreceived", sharedBufferReceivedListener);
    
        try {
            view = await invoke<View>("open_image", path );
            resolveIdPromise(view);
        } catch (e) {
            console.log("fail");
            window.chrome.webview.removeEventListener("sharedbufferreceived", sharedBufferReceivedListener);
        }
    }

    return { views, openImage, setViewSlice };
}

export const viewController = createViewController();
export type ViewController = ReturnType<typeof createViewController>;