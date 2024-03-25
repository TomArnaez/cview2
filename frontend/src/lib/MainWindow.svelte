<script lang="ts">
    import Toolbox from "./components/panels/Toolbox.svelte";
    import Histogram from "./components/charts/Histogram.svelte";
    import Devices from "./components/panels/Devices.svelte";
    import ImageList from "./components/ImageList.svelte";
    import ViewComponent from "./components/ViewComponent.svelte";
    import { viewController} from "./images/viewController";
    import { onMount } from "svelte";
    import type { DrawTool, View, ViewWithBuffer } from "./images/types";
    import CaptureButton from "./components/CaptureButton.svelte";
    
    let view: ViewWithBuffer | null;
    let views: View[] = []
    let drawTool: DrawTool | undefined = undefined;

    onMount(() => {
        viewController.views.subscribe(newViews => {
            if (newViews.length > 0) {
              views = newViews;
            }
        });
    });
</script> 

<div class="flex h-full bg-white">
    <div class="w-116">
      <Devices/>
      <CaptureButton/>
      <button on:click={viewController.openImage}>Test</button>
      <ImageList {views} bind:selectedView={view}/>
    </div>
  
    <div class="flex-grow">
      {#if view}
        <ViewComponent {view} {drawTool} />
      {/if}
    </div>
    
    <div class="w-320">
      {#if view}
        <Toolbox bind:drawTool={drawTool}/>
        <Histogram data={[1, 2, 3, 4, 5]}/>
      {/if}
    </div>
</div>