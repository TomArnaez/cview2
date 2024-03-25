<script lang="ts">
    import type { DrawTool } from "../../images/types";
    import CanvasToolbar from "../CanvasToolbar.svelte";
    import Collapsible from "../Collapsible.svelte";
    import NumberSetting from "../inputs/NumberSetting.svelte";

    export let drawTool: DrawTool;
</script>

<Collapsible title="Geometry">
    <CanvasToolbar bind:drawTool={drawTool}/>
</Collapsible>
<Collapsible title="Zoom"/>
    <NumberSetting icon={"ic:sharp-contrast"}  value={[0.0]} min={-1.0} max={1.0} step={0.1}/>
<Collapsible title="Adjustment">
    <NumberSetting icon={"tabler:brightness"} value={[0.0]} min={-1.0} max={1.0} step={0.1}/>
    <NumberSetting icon={"ic:sharp-contrast"} value={[1.0]} min={0.0} max={1.0} step={0.1}/>
</Collapsible>
<Collapsible title="Annotation"></Collapsible>