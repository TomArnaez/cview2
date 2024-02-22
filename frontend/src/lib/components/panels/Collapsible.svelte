<script lang="ts">
    import { createCollapsible, melt } from '@melt-ui/svelte';

    const {
        elements: { root, content, trigger },
        states: { open },
    } = createCollapsible({
        forceVisible: true,
    });
</script>