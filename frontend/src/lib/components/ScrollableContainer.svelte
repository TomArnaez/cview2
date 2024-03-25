<script lang="ts">
    import { createScrollArea, melt } from '@melt-ui/svelte';
  
    const {
      elements: { root, content, viewport, corner},
    } = createScrollArea({
      type: 'always',
      dir: 'ltr',
    });
  </script>
  
  <div
    use:melt={$root}
    class="relative h-72 w-56 overflow-hidden"
  >
    <div use:melt={$viewport} class="h-full w-full rounded-[inherit]">
      <div use:melt={$content}>
        <slot/>
      </div>
    </div>
    <div use:melt={$corner} />
  </div>