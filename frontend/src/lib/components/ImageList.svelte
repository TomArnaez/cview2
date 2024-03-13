<script lang="ts">
    import type { ImageController } from "../images/imageController";
    import ImageListItem from "./ImageListItem.svelte";
    import type { Image } from "../images/types";
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import ImageListHeader from "./ImageListHeader.svelte";
    import ScrollableContainer from "./ScrollableContainer.svelte";

    export let imageController: ImageController;
    
    const selectedImage = writable<Image | null>(null);
    let images: Image[] = [];

    onMount(() => {
        const unsubscribe = imageController.subscribe(($images) => {
            images = $images;
        });

        return () => {
            unsubscribe();
        };
  });
</script>

<ImageListHeader count={images.length}/>
<ScrollableContainer>
    {#each images as image}
        <button onclick={() => selectedImage.set(image)}>
            <ImageListItem {image} isSelected={true}/>
        </button>
    {/each}
</ScrollableContainer>
