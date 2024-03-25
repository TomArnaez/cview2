<script lang="ts">
    import type { StreamCapture } from "../../detector/types";
    import NumberInput from "../inputs/NumberInput.svelte";

    export let capture: StreamCapture
</script>

<div class="flex items-center justify-between">
    Exposure Time
    <NumberInput value={capture.expTime} min={0} max={500} step={1}></NumberInput>
</div>
