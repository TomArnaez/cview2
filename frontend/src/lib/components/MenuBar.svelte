<script lang="ts">
    import { createMenubar, melt } from "@melt-ui/svelte";

    const {
        elements: { menubar },
        builders: { createMenu }
    } = createMenubar()
    const {
        elements: { menu: fileMenu, item: fileItem, trigger: fileTrigger },
        elements: { menu: editMenu, item: editItem, trigger: editTrigger }
    } = createMenu()
</script>

<div use:melt={$menubar}>
    <button use:melt={$fileTrigger}>Open Menu</button>
    <div use:melt={$fileMenu}>
      <div {...$fileItem} use:fileItem>...</div>
      <div {...$fileItem} use:fileItem>...</div>
      <div {...$fileItem} use:fileItem>...</div>
    </div>

    <button use:melt={$editTrigger}>Edit</button>
    <div use:melt={$editMenu}>
      <div {...$editItem} use:editItem>...</div>
      <div {...$editItem} use:editItem>...</div>
      <div {...$editItem} use:editItem>...</div>
    </div>
</div>