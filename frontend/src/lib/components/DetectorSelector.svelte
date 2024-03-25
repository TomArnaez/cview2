<script lang="ts">
    import { createSelect, melt } from "@melt-ui/svelte";
    import type { Detector } from "../detector/types";

    export let detectors: Detector[]
    
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

<div class="flex flex-col gap-1">
    <button
        class="flex h-10 items-center justify-between rounded-lg bg-gray-100 px-3 py-2
        shadow transition-opacity hover:opacity-90"
        use:melt={$trigger}
        aria-label="Food"
    >
        {$selectedLabel || 'Select a detector'}
    </button>
    {#if $open}
        <div
            class="z-10 flex max-h-[300px] flex-col rounded-lg bg-white p1"
            use:melt={$menu}
        >
            {#each detectors as detector}
                <div 
                    class="relative cursor-pointer rounded-lg py-1"
                    use:melt={$option({ value: detector, label: "detector" })}
                >
                    {detector.specification.width}
                </div>
            {/each}
        </div>
    {/if}
</div>