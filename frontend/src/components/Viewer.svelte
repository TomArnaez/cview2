<script lang="ts">
  import MainWindow from "@cview/components/window/MainWindow.svelte";
  import { createToolState } from "@cview/state-providers/tool";
  import { setContext } from "svelte";

  let tools = createToolState();
  setContext("tools", tools)
</script>

<MainWindow/>