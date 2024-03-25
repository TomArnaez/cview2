<script lang=ts>
    import { createToggleGroup, melt } from "@melt-ui/svelte";
    import type { DrawTool } from "../images/types";

    export let drawTool: DrawTool | undefined;

    function isValidDrawTool(value: string | undefined): value is DrawTool {
        return value === "Rectangle" || value === "Line" || value === undefined;
    }

    const {
        elements: { root, item },
        states: { value }
    } = createToggleGroup();

    const toggleItems = [
        { name: 'Line' },
        { name: 'Rectangle' }
   ];

   value.subscribe(tool => {
        if (typeof tool === "string" && isValidDrawTool(tool)) {
            drawTool = tool 
        }
   })
</script>

<div class="flex items-center" use:melt={$root}>
    {#each toggleItems as toggleItem (toggleItem.name)}
        <button use:melt={$item(toggleItem.name)} class="text-red-500 data-[state=on]:bg-blue-300">
            {toggleItem.name}
        </button>
    {/each}
</div>
