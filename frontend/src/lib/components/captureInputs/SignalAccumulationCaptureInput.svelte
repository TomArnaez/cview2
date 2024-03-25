<script lang="ts">
    import type { SignalAccumulationCapture } from "../../detector/types";
    import NumberInput from "../inputs/NumberInput.svelte";

    export let capture: SignalAccumulationCapture
</script>

<div class="flex items-center justify-between">
    Frames
    <NumberInput value={capture.frames} min={0} max={500} step={1}></NumberInput>
</div>
