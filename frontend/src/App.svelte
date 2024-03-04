<script lang="ts">
  import "./styles.css";
  import MainWindow from "./lib/MainWindow.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  onMount(async () => {
    const unlisten = await listen('image-manager-state-changed', (event) => {
      const imageState: ImageDetails[] = event.payload;
      console.log(imageState);
    })
    await invoke("init");
    await invoke("test_cmd");
    
    console.log(await commands.deleteImage(0));
    window.chrome.webview.addEventListener("sharedbufferreceived", e => {
      console.log(new Uint32Array(e.sharedBuffer));
    })
  });
</script>
  
<MainWindow/>