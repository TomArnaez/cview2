<script lang="ts">
  import "./styles.css";
  import MainWindow from "./lib/MainWindow.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { detectorStore } from "./lib/detector/detector";
  import type { Detector } from "./lib/detector/types";
    import { listen } from "@tauri-apps/api/event";

  onMount(async () => {
    await invoke("init");
    const store = detectorStore;
    detectorStore.set(await invoke<Detector[]>("list_all_detectors"))
  });
  
</script>
  
<MainWindow/>