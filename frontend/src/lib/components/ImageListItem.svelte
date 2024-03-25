<script lang="ts">
    import type { View } from "../images/types";

    export let view: View;
    export let isSelected: boolean;
</script>
<div class="{isSelected ? 'bg-blue-300' : 'bg-neutral-50'} border border-black">
    width: {view.height}, height: {view.width}
</div>