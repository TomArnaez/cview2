<script lang="ts">
    import ImageListItem from "./ImageListItem.svelte";
    import ImageListHeader from "./ImageListHeader.svelte";
    import ScrollableContainer from "./ScrollableContainer.svelte";
    import type { View } from "../images/types";

    export let selectedView: View | null;
    export let views: View[]
</script>

<ImageListHeader count={views.length}/>
<ScrollableContainer>
    {#each views as view, index}
        <div on:click={() => {selectedView = view}}>
            <ImageListItem {view} isSelected={view == selectedView}/>
        </div>
    {/each}
</ScrollableContainer>