import "./styles.css";
import App from "./App.svelte";
import { invoke } from "@tauri-apps/api/core";

invoke('create_shared_buffer', { size: 5000*5000 })
chrome.webview.releaseBuffer


const app = new App({
  target: document.getElementById("app"),
});

export default app;
