<script lang="ts">
    export let value: number;
    export let min: number;
    export let max: number;
    export let step: number;
</script>

<input
    type="number"
    min={min}
    max={max}
    step={step}
    id="numberInput"
    bind:value={value}
    class="form-input border border-gray-300shadow-sm"
/>