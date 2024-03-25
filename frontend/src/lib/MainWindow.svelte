<script lang="ts">
    import Toolbox from "./components/panels/Toolbox.svelte";
    import Histogram from "./components/charts/Histogram.svelte";
    import ImageList from "./components/ImageList.svelte";
    import ViewComponent from "./components/ViewComponent.svelte";
    import { viewController } from "./images/viewController";
    import { detectorService } from "./detector/detectorService";
    import { onMount } from "svelte";
    import type { DrawTool, View, ViewWithBuffer } from "./images/types";
    import Capture from "./components/Capture.svelte";
    import type { Detector } from "./detector/types";
    import DetectorSelector from "./components/DetectorSelector.svelte";
    
    let view: ViewWithBuffer | null;
    let views: View[] = []
    let detectors: Detector[] = [];
    let drawTool: DrawTool | undefined = undefined;

    onMount(() => {
        viewController.views.subscribe(newViews => {
            if (newViews.length > 0) {
              views = newViews;
            }
        });

        detectorService.subscribe(newDetectors => {
          detectors = newDetectors
          console.log(detectors)
        })
    });
</script> 

<div class="flex h-full bg-white">
  <div class="w-64 h-full border-r border-gray-300 flex-col p-2 gap-2">
    <DetectorSelector {detectors}/>
    <Capture/>
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