<script lang="ts">
    import { createSelect, melt } from "@melt-ui/svelte"
  
    export let name: string
    export let labels: string[];
  
    const {
      elements: { trigger, menu, option },
      states: { selectedLabel, open },
    } = createSelect<string>({
      forceVisible: true,
      positioning: {
        placement: 'bottom',
        fitViewport: true,
        sameWidth: true,
      },
    });
</script>

<div class="flex items-center">
    {name}: 
    <div class="flex-1 flex-col gap-1">
        <button
            class="flex h-10 min-w-[220px] items-center justify-between px-3 py-2
            text-magnum-700 shadow transition-opacity hover:opacity-70"
            use:melt={$trigger}
        >
            {$selectedLabel || 'Select a flavor'}
        </button>
        {#if $open}
            <div
            class=" z-10 flex max-h-[300px] flex-col
            overflow-y-auto bg-white p-1
            shadow focus:!ring-0"
            use:melt={$menu}
            >
            {#each labels as item}
                <div
                class="relative cursor-pointer py-1 pl-8 pr-4 text-neutral-800
                focus:z-10
                data-[highlighted]:bg-magnum-200 data-[highlighted]:text-magnum-900
                data-[disabled]:opacity-50"
                use:melt={$option({ value: item, label: item })}
                >
                {item}
                </div>
            {/each}
            </div>
        {/if}
    </div>
</div>