<script lang="ts">
    import Canvas from "./Canvas.svelte";
    import type { DrawTool, ViewWithBuffer } from "../images/types";
    import { invoke } from "../backend/ipc";

    export let view: ViewWithBuffer;
    export let drawTool: DrawTool;

    $: imageData = new ImageData(view.buffer, view.width, view.height);

    async function handleMouseMove(e: CustomEvent<any>) {
        if (view) {
            console.log(e);
            let value = await invoke<number>("get_pixel_value", { id: view.id, x: e.detail.x, y: e.detail.y})
            console.log(value);
        }
    }
</script>

<div class="relative w-full h-full">
    <Canvas on:mouseMove={handleMouseMove} {imageData} {drawTool} />
    <!-- <div class="absolute top-0 left-0 w-full h-full flex pointer-events-none">
        TextDecoderStream
    </div> -->
</div>