<script lang="ts">
    import Badge from "./Badge.svelte";

    export let count: number | undefined;
</script>

<div class="flex border-b-2 border-gray-300">
    <span class="text-base font-semibold">Image Stacks</span>

    {#if count !== undefined}
        <Badge {count} />
    {/if}
</div>