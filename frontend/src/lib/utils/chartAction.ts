import type {  ChartConfiguration } from 'chart.js';
import Chart from 'chart.js/auto';

export function chartAction(node: HTMLCanvasElement, chartConfig: ChartConfiguration): {
    update: (newConfig: ChartConfiguration) => void;
    destroy: () => void;
} {
    let chart: Chart | null = null;

    function createChart(): void {
        if (!Chart) {
            console.error("Chart.js is not available");
            return;
        }
    }

    chart = new Chart(node, chartConfig);

    createChart();

    return {
        update(newConfig: ChartConfiguration): void {
            if (chart) {
                chart.destroy();
                chartConfig = newConfig;
                createChart();
            }
        },
        destroy(): void {
            chart?.destroy();
            chart = null;
        }
    }
}