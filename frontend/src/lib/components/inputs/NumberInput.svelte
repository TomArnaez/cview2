<script lang="ts">
    import { getContext } from "svelte";
    import type { Writable } from "svelte/store";

    export let min: number;
    export let max: number;
    export let step: number;
    
    const sharedValue: Writable<number[]> = getContext('sharedValue');

    let numberValue: number;

    sharedValue.subscribe(value => {
        numberValue = value[0];
    });
    
</script>
  
<input
    type="number"
    min={min}
    max={max}
    step={step}
    id="numberInput"
    bind:value={numberValue}
    class="form-input border border-gray-300 bg-gray-600 text-gray-700 shadow-sm"
/>