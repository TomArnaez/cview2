<script lang="ts">
    import type { ImageController } from "../images/imageController";
    import ImageListItem from "./ImageListItem.svelte";
    import { onMount } from 'svelte';
    import type { Image } from "../images/types";

    export let imageController: ImageController;
    let images: Image[] = [];
</script>

{#each images as image}
    <ImageListItem {image}/>
{/each}
