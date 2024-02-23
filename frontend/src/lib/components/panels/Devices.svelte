<script lang="ts">
  import type { CaptureMode } from "../../../bindings";
  import Collapsible from "../Collapsible.svelte";
  import SelectButton from "../inputs/SelectButton.svelte";

  const devices = [ "PLEORA", "USB", "S2I" ]

  let captureMode: CaptureMode;
</script>

<Collapsible title="Devices">
    <SelectButton name={"Detector"} labels={devices}/>
</Collapsible>