<script lang="ts">
    import { onMount } from "svelte";
    import { createStaticImageView, type IView } from "../images/types";
    import Canvas from "./canvas/Canvas.svelte";

    export let view: IView;
    let imageData: ImageData;

    onMount(async () => {
        view = await createStaticImageView("test");
        imageData = view.getImageData();
    });

    $: imageData
</script>

<Canvas {imageData}/>