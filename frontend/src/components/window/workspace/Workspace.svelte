<script lang="ts">
  import { onMount } from "svelte";
  import ControlSidebar from "./ControlSidebar.svelte";

  let viewport: HTMLElement;

  onMount(() => {
    const resizeObserver = new ResizeObserver(entries => {
      for (let entry of entries) {
        console.log('Size changed', entry.contentRect);
      }
    });

    resizeObserver.observe(viewport);

    return () => {
      resizeObserver.disconnect();
    };
  });
</script>

<div class="flex-1 flex overflow-auto">
    <div class="w-1/6">
        <ControlSidebar/>
    </div>
  
    <div bind:this={viewport} class="flex-1 bg-gray-700">
      Viewport
    </div>
  </div>