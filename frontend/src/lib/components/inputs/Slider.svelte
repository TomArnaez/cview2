<script lang="ts">
    import { createSlider, melt } from '@melt-ui/svelte';
    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';

    const sharedValue: Writable<number[]> = getContext('sharedValue');

    export let min: number;
    export let max: number;
    export let step: number;

    const {
      elements: { root, range, thumbs },
    } = createSlider({
      value: sharedValue,
      min: min,
      max: max,
      step: step,
    });
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