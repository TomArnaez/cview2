<script lang="ts">
    export let value: number;
    export let min: number = 0;
    export let max: number = 100;
    export let step: number = 1;
    export let label: string;
    export let id: string;

    interface SliderChangeEventDetail {
        value: number;
    }

    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher<SliderChangeEventDetail>();

    function handleChange(event: Event) {
        const target = event.target as HTMLInputElement; // Type assertion
        value = parseFloat(target.value);
        console.log(value);
    }

</script>

<div class="flex flex-col">
    {#if label}
    <label for={id} class="text-sm font-medium text-gray-700">{label}</label>
    {/if}
    <input
      type="range"
      class="slider"
      id={id}
      min={min}
      max={max}
      step={step}
      bind:value
      on:change={handleChange}
    />
  </div>