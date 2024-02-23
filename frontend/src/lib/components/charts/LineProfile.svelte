<script lang="ts">
    import { chartAction } from "../../utils/chartAction";
    import type { ChartConfiguration } from 'chart.js';

    export let data: number[] = [];

    let chartConfig: ChartConfiguration;
    $: chartConfig = {
        type: "line",
        data: {
            datasets: [{
                data: data,
                fill: "false"
            }]
        }
    }
</script>

<canvas use:chartAction={chartConfig}></canvas>