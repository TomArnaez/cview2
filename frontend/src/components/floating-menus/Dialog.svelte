<script lang="ts">
    import { getContext, onMount } from "svelte";
    import { DialogState } from "/src/state-providers/dialog";
    import IconLabel from "../widgets/labels/IconLabel.svelte";

    const dialog = getContext<DialogState>("dialog");
</script>

<dialog open class="rounded-lg shadow-xl">
    <!-- Header -->
    <div class="flex justify-between items-center bg-blue-500 text-white p-4 rounded-t-lg">
        <h2 class="text-lg">Dialog Header</h2>
    </div>

    <!-- Content -->
    <div class = "p-4">
        {#if $dialog.panicDetails}

        {/if}
    </div>
</dialog>