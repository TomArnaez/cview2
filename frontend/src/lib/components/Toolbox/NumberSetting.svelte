<script lang="ts">
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';
    import Icon from "@iconify/svelte";
    import Slider from "../../components/inputs/Slider.svelte";
    import NumberInput from "./../inputs/NumberInput.svelte";

    export let icon: string;
    export let initial: number;
    export let min: number;
    export let max: number;
    export let step: number;

    const sharedValue = writable([initial]);
    setContext('sharedValue', sharedValue);
</script>

<div class="flex">
    <Icon icon={icon}/>
    <NumberInput min={min} max={max} step={step}/>
    <Slider min={min} max={max} step={step}/>
</div>