<script lang="ts">
    import Toolbox from "./components/panels/Toolbox.svelte";
    import Histogram from "./components/charts/Histogram.svelte";
    import Devices from "./components/panels/Devices.svelte";
    import ImageList from "./components/ImageList.svelte";
    import ViewComponent from "./components/ViewComponent.svelte";
    import { createImageController } from "./images/imageController";

    const imageController = createImageController();
</script> 

<div class="flex h-full bg-gray-800">
    <div class="w-32">
      <ImageList {imageController}/>
    </div>
  
    <div class="flex-grow">
    </div>
    
    <div class="w-320">
      <Devices/>
      <Toolbox/>
      <Histogram data={[1, 2, 3, 4, 5]}/>
    </div>
</div>