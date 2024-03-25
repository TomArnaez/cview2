<script lang="ts">
    import { createCollapsible, melt } from "@melt-ui/svelte";
    import SelectButton  from "./inputs/SelectButton.svelte"
    import SignalAccumulationInput from "./captures/SignalAccumulationInput.svelte";
    import { slide } from "svelte/transition";
    import type { SignalAccumulationCapture } from "../detector/types";

    const {
        elements: { root, content, trigger },
        states: { open},
    } = createCollapsible({
    });

    let name = "Capture Mode";
    let labels = ["Stream", "Smart Capture"]
    let signalCapture: SignalAccumulationCapture = {
        expTimes: [],
        frames: 5
    };
</script>

<div use:melt={$root}>
    <button 
        use:melt={$trigger}
        class="bg-blue-500 hover:bg-blue-600 text-white py-2 px-6 rounded-sm"
    >
        Capture
    </button>

    <SignalAccumulationInput capture={signalCapture}/>

    <div>
        {#if open}
            <div use:melt={$content} transition:slide>
                <SelectButton {name} {labels}/>
            </div>
        {/if}
    </div>
</div>