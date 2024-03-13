<script lang="ts">
    import Toolbox from "./components/panels/Toolbox.svelte";
    import Histogram from "./components/charts/Histogram.svelte";
    import Devices from "./components/panels/Devices.svelte";
    import ImageList from "./components/ImageList.svelte";
    import { createImageController, type ImageController } from "./images/imageController";

    let imageController: ImageController = createImageController();
    imageController.openImage();
</script> 

<div class="flex h-full bg-gray-800">
    <div class="w-32">
      <ImageList {imageController} />
    </div>
  
    <div class="flex-grow">
    </div>
    
    <div class="w-320">
      <Devices/>
      <Toolbox/>
      <Histogram data={[1, 2, 3, 4, 5]}/>
    </div>
</div>