type EventName = keyof HTMLElementEventMap | keyof WindowEventHandlersEventMap | "modifyinputfield";
type EventListenerTarget = {
	addEventListener: typeof window.addEventListener;
	removeEventListener: typeof window.removeEventListener;
};

export function createInputManager() {

    // Event listeners

	const listeners: { target: EventListenerTarget; eventName: EventName; action: (event: any) => void; options?: AddEventListenerOptions }[] = [
		{ target: window, eventName: "keyup", action: (e: KeyboardEvent) => onKeyUp(e) },
		{ target: window, eventName: "keydown", action: (e: KeyboardEvent) => onKeyDown(e) },
		{ target: window, eventName: "pointermove", action: (e: PointerEvent) => onPointerMove(e) },
		{ target: window, eventName: "pointerdown", action: (e: PointerEvent) => onPointerDown(e) },
		{ target: window, eventName: "pointerup", action: (e: PointerEvent) => onPointerUp(e) },
		{ target: window, eventName: "mousedown", action: (e: MouseEvent) => onMouseDown(e) },
		{ target: window, eventName: "wheel", action: (e: WheelEvent) => onWheelScroll(e), options: { passive: false } },
		{ target: window.document, eventName: "contextmenu", action: (e: MouseEvent) => onContextMenu(e) },
	];

    // Event bindings

    function bindListeners() {
		// Add event bindings for the lifetime of the application
		listeners.forEach(({ target, eventName, action, options }) => target.addEventListener(eventName, action, options));
	}
	function unbindListeners() {
		// Remove event bindings after the lifetime of the application (or on hot-module replacement during development)
		listeners.forEach(({ target, eventName, action, options }) => target.removeEventListener(eventName, action, options));
	}

    // Keyboard events

    async function shouldRedirectKeyboardEvent(e: KeyboardEvent): Promise<boolean> {
        const key = e.key;

        const accelKey = e.ctrlKey;

        // Don't redirect paste
        if (key === "KeyV" && accelKey) return false;

        // Don't redict a fullscreen request
        if (key === "F11" && e.type === "keydown" && !e.repeat) {
			e.preventDefault();
			return false;
		}

        // Don't redirect a reload request
        if (key === "F5") return false;
		if (key === "KeyR" && accelKey) return false;

        // Don't redirect debugging tools
		if (["F12", "F8"].includes(key)) return false;
		if (["KeyC", "KeyI", "KeyJ"].includes(key) && accelKey && e.shiftKey) return false;

        return true;
    }

    async function onKeyDown(e: KeyboardEvent) {
        const key = e.key;

        const NO_KEY_REPEAT_MODIFIER_KEYS = ["ControlLeft", "ControlRight", "ShiftLeft", "ShiftRight", "MetaLeft", "MetaRight", "AltLeft", "AltRight", "AltGraph", "CapsLock", "Fn", "FnLock"];
        if (e.repeat && NO_KEY_REPEAT_MODIFIER_KEYS.includes(key)) return;

        if (await shouldRedirectKeyboardEvent(e)) {
            e.preventDefault();
            return;
        }
    }

    async function onKeyUp(e: KeyboardEvent) {
        const key = e.key;

        if (await shouldRedirectKeyboardEvent(e)) {
            e.preventDefault();
        }
    }

    // Pointer events

    async function onPointerMove(e: PointerEvent) {

    }

    function onMouseDown(e: MouseEvent) {

    }

    function onPointerDown(e: PointerEvent) {

    }

    function onPointerUp(e: PointerEvent) {

    }

    // Mouse events

    function onWheelScroll(e: WheelEvent) {

    }

    function onContextMenu(e: MouseEvent) {

    }

    // Window events
}