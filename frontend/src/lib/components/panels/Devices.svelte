<script lang="ts">
  import type { Detector } from "../../detector/types";
  import Collapsible from "../Collapsible.svelte";
  import DetectorSelector from "../DetectorSelector.svelte";
  import SelectButton from "../inputs/SelectButton.svelte";

  export let detectors: Detector[]
</script>

<Collapsible title="Devices">
</Collapsible>