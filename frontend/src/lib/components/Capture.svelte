<script lang="ts">
    import { createSelect, melt } from "@melt-ui/svelte";
    import { fade } from 'svelte/transition';
    import { Circle } from "svelte-loading-spinners";
    import StreamCaptureInput from "./captureInputs/StreamCaptureInput.svelte";
    import SignalAccumulationCaptureInput from "./captureInputs/SignalAccumulationCaptureInput.svelte";
    import type { CaptureMode } from "../detector/types";

    const {
        elements: { trigger, menu, option },
        states: { open, selectedLabel }
    } = createSelect({});
    
    const captureModes = [
        {name: "Stream"},
        {name: "Signal"},
    ];

    let captureComponent: typeof StreamCaptureInput | typeof SignalAccumulationCaptureInput | null = null;

    selectedLabel.subscribe(label => {
        if (label === "Stream") {
            captureComponent = StreamCaptureInput;
            capture = {
                type: "StreamCapture",
                expTime: 100
            }
        } else if (label === "Signal") {
            captureComponent = SignalAccumulationCaptureInput;
            capture = {
                type: "SignalAccumulationCapture",
                expTimes: [100, 200, 300],
                frames: 10
                
            }
        }
    });

    let capture: CaptureMode | null = null;
    let isCapturing: boolean = true;
</script>

<div class="flex flex-col gap-2">
    <div class="inline-flex h-15">
        <button 
            class="bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 border-r-2 border-blue-600 flex-1"
        >
            {#if isCapturing}
                <div class="flex items-center gap-2">
                    <Circle size={20} color={"white"}/>
                    Capturing...
                </div>
            {:else}
                Capture
            {/if}
        </button>
        <button 
            class="bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 flex-1"
            use:melt={$trigger}
        >
            {$selectedLabel}
        </button>
        {#if open}
            <div
                class="z-1 flex flex-col bg-white rounded-lg shadow p-1"
                transition:fade={{ duration: 150 }}
                use:melt={$menu}
            >
                {#each captureModes as item}
                    <div
                        class="relative cursor-pointer rounded-lg
                        hover:bg-gray-300"
                        use:melt={$option({ label: item.name})}
                    >
                        {item.name}
                    </div>
                {/each}
            </div>
        {/if}
    </div>
    {#if captureComponent != null && capture != null}
        <svelte:component this={captureComponent} bind:capture={capture} />
    {/if}
</div>