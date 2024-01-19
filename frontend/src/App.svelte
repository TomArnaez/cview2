<script lang="ts">
  import { onMount } from "svelte";
  import Viewer from "./components/Viewer.svelte";
  import { commands, events, type Message, type ToolMessage, type ToolType } from "./communication/bindings";

  onMount(async () => {
    const toolMessage: ToolMessage = {
            ActivateTool: { tool_type: "Rectangle" as ToolType }
        };
    const message: Message = {
        Tool: toolMessage
    };

    await commands.init();
    events.frontendEvent.listen(e => {
      console.log("Got event", e);
    });
  });
</script>

<Viewer/>