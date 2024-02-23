<script lang="ts">
    import { createSlider, createSync, melt } from '@melt-ui/svelte';

    
    export let value: number[];
    export let min: number;
    export let max: number;
    export let step: number;

    const {
      elements: { root, range, thumbs, },
      states,
    } = createSlider({
      min: min,
      max: max,
      step: step,
    });

    const sync = createSync(states)
    $: sync.value(value, (v) => (value = v))
</script>
  
  <span use:melt={$root} class="relative flex h-[20px] w-[200px] items-center">
    <span class="h-[3px] w-full bg-black/40">
      <span use:melt={$range} class="h-[3px] bg-white" />
    </span>
  
    <span
      use:melt={$thumbs[0]}
      class="h-5 w-5 rounded-full bg-white focus:ring-4 focus:!ring-black/40"
    />
  </span>