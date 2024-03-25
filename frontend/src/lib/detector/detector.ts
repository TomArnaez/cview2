import { writable } from "svelte/store";
import { listen } from "../backend/ipc";
import type { Detector } from "./types";

export const detectorStore = writable<Detector[]>([]);

