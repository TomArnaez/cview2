import { commands } from "../bindings.ts"

export class TauriHandle {
    /**
     */
    async initAfterFrontendReady() {
        await commands.initAfterFrontendReady();
    }

    /**
     * Mouse movement with the screenspace of bounds of the viewport
     * @param {number} x
     * @param {number} y
     * @param {number} mouse_keys
     * @param {number} modifiers
     */
    async onMouseMove(x: number, y: number, mouse_keys: number, modifier: number) {
        await commands.onMouseMove(x, y, mouse_keys, modifier);
    }

    /**
     * Mouse scrolling within the screenspace of the bounds of the viewport
     * @param {number} x
     * @param {number} y
     * @param {number} mouse_keys
     * @param {number} wheel_delta_x
     * @param {number} wheel_delta_y
     * @param {number} wheel_delta_z
     * @param {number} modifiers
     */
    async onWheelScroll(x: number, y: number, mouse_keys: number, wheel_delta_x: number, wheel_delta_y: number, wheel_delta_z: number, modifiers: number) {
        await commands.onWheelScroll( x, y,  mouse_keys, wheel_delta_x, wheel_delta_y,  wheel_delta_z, modifiers);
      }
}