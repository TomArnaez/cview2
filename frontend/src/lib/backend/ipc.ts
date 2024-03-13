import { invoke as invokeTauri } from "@tauri-apps/api/core";
import { listen as listenTauri, type EventName, type EventCallback } from "@tauri-apps/api/event";

export async function invoke<T>(command: string, params: Record<string, unknown> = {}): Promise<T> {
    return (
        invokeTauri<T>(command, params)
            .then((value) => {
                return value;
            })
			.catch((reason) => {
				console.error(`ipc->${command}: ${JSON.stringify(params)}`, reason);
				throw Error("ipc failed");
			})
    )
}

export function listen<T>(event: EventName, handle: EventCallback<T>) {
    const unlisten = listenTauri(event, handle);
	return () => unlisten.then((unlistenFn) => unlistenFn());
}