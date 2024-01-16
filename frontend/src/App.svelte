<script lang="ts">
  import { onMount } from "svelte";
  import Viewer from "./components/Viewer.svelte";

  onMount(async () => {
    commands.init_after_frontend_ready();
  });
</script>



<Viewer/>