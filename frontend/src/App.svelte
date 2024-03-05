<script lang="ts">
  import "./styles.css";
  import MainWindow from "./lib/MainWindow.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  onMount(async () => {
    await invoke("init");
    
    window.chrome.webview.addEventListener("sharedbufferreceived", e => {
      console.log(new Uint32Array(e.sharedBuffer));
    })
  });
</script>
  
<MainWindow/>