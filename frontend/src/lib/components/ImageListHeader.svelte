<script lang="ts">
  import { viewController } from "../images/viewController";
    import Badge from "./Badge.svelte";

    export let count: number | undefined;
</script>

<div class="flex justify-between items-center border-t-2 border-gray-300">
    <div class="flex items-center">
        <span class="text-base font-semibold">Image Stacks</span>
        {#if count !== undefined}
            <Badge {count} />
        {/if}
    </div>
    <div>
        <button class="bg-gray-300 hover:bg-gray-400 text-gray-800 py-1 px-2 text-sm rounded" on:click={viewController.openImage}>Open</button>
    </div>
</div>