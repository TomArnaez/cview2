<script lang="ts">
    import { createCheckbox, melt } from "@melt-ui/svelte";
    const {
    elements: { root, input },
  } = createCheckbox({
    defaultChecked: false,
  });

  export let labelText: string;
</script>

<div class="flex items-center justify-center">
    <button use:melt={$root}>
    </button>
    <label class="pl-4 font-medium" for="checkbox">
        {labelText}
    </label>
</div>