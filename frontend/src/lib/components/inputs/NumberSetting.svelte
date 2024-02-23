<script lang="ts">
    import Icon from "@iconify/svelte";
    import Slider from "../../components/inputs/Slider.svelte";
    import NumberInput from "./../inputs/NumberInput.svelte";

    export let icon: string;
    export let min: number;
    export let max: number;
    export let step: number;
    export let value: number[]
</script>

<div class="flex">
    <Icon icon={icon}/>
    <NumberInput min={min} max={max} step={step} bind:value={value[0]}/>
    <Slider min={min} max={max} step={step} bind:value/>
</div>