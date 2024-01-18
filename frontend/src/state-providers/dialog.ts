import { writable } from "svelte/store";

import type { IconName } from "@cview/utility-functions/icons";
import { events } from "@cview/communication/bindings"

export function createDialogState() {
    const { subscribe, update } = writable({
		visible: false,
		title: "",
		icon: "" as IconName,
        // Special case for the crash dialog because we cannot handle button widget callbacks from Rust once the editor instance has panicked
		panicDetails: "",

	});

    function dismissDialog() {
        update((state) => {
            return state;
        })
    }

    // Creates a crash dialog from JS once the editor has panicked.
	// Normal dialogs are created in the Rust backend, but for the crash dialog, the editor instance has panicked so it cannot respond to widget callbacks.
	function createCrashDialog(panicDetails: string) {
		update((state) => {
			state.visible = true;
            
			state.title = "Crash";
			state.panicDetails = panicDetails;

			return state;
		});
	}

    const listen = events.frontendEvent.listen((e) => {

    })

    return {
		subscribe,
		dismissDialog,
		createCrashDialog,
	};
}

export type DialogState = ReturnType<typeof createDialogState>;