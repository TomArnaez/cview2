import { TauriHandle } from "./tauri-editor";

let tauriHandle: TauriHandle | undefined;

export async function initTauri(): Promise<void> {
    // Skip if the Tauri module is already initialised
    if (tauriHandle !== undefined)

    tauriHandle = new TauriHandle();
}

export async function getTauriHandle(): Promise<TauriHandle | undefined> {
    return tauriHandle;
}