<script lang="ts">
  import { getContext } from 'svelte';
  import { derived } from 'svelte/store';

  // Unique ID for each button instance
  let id = Math.random().toString(36).substring(2, 9);
  export let label = 'Button'; // Optional: for button labeling

  // Consume the context provided by ToggleGroup
  const { activeButtonId, setActiveButtonId } = getContext('toggleGroup');

  // Derived store to compute the active state of the button
  const isActive = derived(activeButtonId, $activeButtonId => $activeButtonId === id);

  function handleClick() {
    setActiveButtonId(id);
  }
</script>

<button on:click={handleClick} class="transition-colors duration-150 ease-in-out p-2 text-white font-bold rounded { $isActive ? 'bg-blue-500' : 'bg-gray-200'}">
  {label}
</button>