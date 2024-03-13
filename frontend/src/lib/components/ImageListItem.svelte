<script lang="ts">
    import type { ImageDetails } from "../../bindings";

    export let image: ImageDetails;
    export let isSelected: boolean
</script>

{#if isSelected}
    height: {image.height}, width: {image.width}
{:else}
    Bye
{/if}
