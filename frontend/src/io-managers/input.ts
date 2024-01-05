type EventListenerTarget = {
    addEventListener: typeof window.addEventListener;
    removeEventListener: typeof window.removeEventListener;
}
type EventName = keyof HTMLElementEventMap


export function createInputManager(): void {
    const listeners: { target: EventListenerTarget, eventName: EventName; action: (event: any) => void; }[] = [
        { target: window, eventName: "resize", action: () => onWindowResize(window.document.body)},
        { target: window, eventName: "wheel", action: (e: WheelEvent) => onWheelScroll(e)},
        { target: window, eventName: "contextmenu", action(e: MouseEvent) => onContextMenu(e) },
    ]

    // Window events

    function onWindowResize(container: HTMLElement) {

    }

    function onWheelScroll(e: WheelEvent) {
        const { target } = e;
    }

    function onContextMenu(e: MouseEvent) {

    }
}