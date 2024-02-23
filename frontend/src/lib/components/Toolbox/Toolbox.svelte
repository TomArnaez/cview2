<script lang="ts">
    import Collapsible from "./Collapsible.svelte";
    import NumberSetting from "./NumberSetting.svelte";
</script>

<Collapsible title="Geometry">
</Collapsible>
<Collapsible title="Zoom"/>
    <NumberSetting icon={"ic:sharp-contrast"}  initial={0.0} min={-1.0} max={1.0} step={0.1}/>
<Collapsible title="Adjustment">
    <NumberSetting icon={"tabler:brightness"} initial={0.0} min={-1.0} max={1.0} step={0.1}/>
    <NumberSetting icon={"ic:sharp-contrast"} initial={1.0} min={0.0} max={1.0} step={0.1}/>
</Collapsible>
<Collapsible title="Annotation"></Collapsible>