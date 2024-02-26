         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
async init() : Promise<null> {
return await TAURI_INVOKE("plugin:tauri-specta|init");
},
async test() : Promise<__Result__<null, { uuid: string; len: number; _marker: null }>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:tauri-specta|test") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async runCapture(detectorId: string, capture: CaptureMode) : Promise<__Result__<null, null>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:tauri-specta|run_capture", { detectorId, capture }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}



/** user-defined types **/

export type CaptureMode = { Stream: StreamCapture } | { Sequence: SequenceCapture }
export type CaptureSettings = { dds_on: boolean; full_well_mode: FullWellModes; roi: ROI; test_mode: boolean; timeout: Duration }
export type Duration = { secs: number; nanos: number }
export type FullWellModes = "Low" | "High" | "Unknown"
export type ROI = { x: number; y: number; w: number; h: number }
export type SequenceCapture = { acquisition_settings: CaptureSettings; num_frames: number; exposure_time: Duration }
export type StreamCapture = { capture_settings: CaptureSettings; stream_time: Duration | null }

/** tauri-specta globals **/

         import { invoke as TAURI_INVOKE } from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type Window as __WebviewWindowHandle__ } from "@tauri-apps/api/window";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

type __Result__<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindowHandle__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindowHandle__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}

     