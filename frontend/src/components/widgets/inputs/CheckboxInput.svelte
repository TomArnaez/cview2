<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let checked = false;
    export let disabled = false;
	export let tooltip: string | undefined = undefined;

    export function isChecked() {
        return checked;
    }
</script>

<style>
</style>