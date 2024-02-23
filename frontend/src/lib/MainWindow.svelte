<script lang="ts">
    import Toolbox from "./components/panels/Toolbox.svelte";
    import Histogram from "./components/charts/Histogram.svelte";
    import Viewer from "./components/viewer/Viewer.svelte";
    import Devices from "./components/panels/Devices.svelte";
</script> 

<div class="flex h-full bg-gray-800">
    <div class="w-32">Left Fixed</div>
  
    <div class="flex-grow">
      <Viewer/>
    </div>
    
    <div class="w-320">
      <Devices/>
      <Toolbox/>
      <Histogram data={[1, 2, 3, 4, 5]}/>
    </div>
</div>