<script lang="ts">
    import Konva from "konva";
    import { onMount } from "svelte";

    let stage: Konva.Stage;
    let containerWidth: number;
    let containerHeight: number;

    onMount(() => {
        stage = new Konva.Stage({
            container: "container",
            width: containerWidth,
            height: containerHeight
        });
    })
</script>

<div id="container" class="w-full h-full bg-red-800" bind:clientHeight={containerHeight} bind:clientWidth={containerWidth} />