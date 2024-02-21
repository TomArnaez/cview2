<script lang="ts">
  import Router from "svelte-spa-router";
  import routes from "./routes";
  import { AppShell } from '@skeletonlabs/skeleton';
  import Navbar from "./components/Navbar.svelte";
</script> 

<AppShell>
	<svelte:fragment slot="header">Header</svelte:fragment>
	<svelte:fragment slot="sidebarLeft"><Navbar/></svelte:fragment>
	<svelte:fragment slot="sidebarRight">Sidebar Right</svelte:fragment>
	<slot />
	<svelte:fragment slot="pageFooter">Page Footer</svelte:fragment>
</AppShell>