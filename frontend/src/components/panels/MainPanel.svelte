<script lang="ts">
    export let title: string;
    export let open = true;
    export let collapsible = true;

    function toggle() {
        if (collapsible) {
            open = !open;
        }
    }
</script>

<div class="bg-gray-500 text-white rounded-lg overflow-hidden">
    <button class="flex justify-between items-center w-full px-4 py-2 text-left" on:click={toggle}>
        <span>{title}</span>
        {#if collapsible}
            <span>
                {open ? '▲' : '▼'}
            </span>
        {/if}
    </button>
    {#if open}
        <div class="p-4">
            <slot/>
        </div>
    {/if}
</div>