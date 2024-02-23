<script lang="ts">
    import { createCollapsible, melt } from "@melt-ui/svelte"
    import Icon from '@iconify/svelte';

    export let title: string

    const {
      elements: { root, content, trigger },
      states: { open },
    } = createCollapsible({
      forceVisible: true,
    });
</script> 

<div use:melt={$root} class="w-full text-gray-100">
    <div class="bg-gray-600 ring-1 ring-black">
        <button use:melt={$trigger}>
            {#if $open}
            <Icon icon="ph:caret-down"/>
            {:else}
            <Icon icon="ph:caret-right"/>
            {/if}
        </button>
        <span>{title}</span>
    </div>
    {#if $open}
    <div use:melt={$content}>
        <slot/>
    </div>
    {/if}
</div>