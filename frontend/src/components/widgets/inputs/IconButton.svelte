<script lang="ts">
    import { type IconName, type IconSize } from "@cview/utility-functions/icons";
    import IconLabel from "@cview/components/widgets/labels/IconLabel.svelte";

    export let icon: IconName;
    export let size: IconSize;
    export let disabled = false;
    export let active = false;
    export let tooltip: string | undefined = undefined;

    export let action: (e?: MouseEvent) => void;
</script>

<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" 
        on:click={action} 
        disabled={disabled}
        aria-label={tooltip}
        class:active={active}>
    <IconLabel {icon} />
</button>