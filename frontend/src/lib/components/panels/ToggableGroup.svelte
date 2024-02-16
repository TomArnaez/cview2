<script lang="ts">
    import { setContext, onDestroy } from 'svelte';
    import { writable, type Writable } from 'svelte/store';
  
    // Define a store to keep track of the active button ID
    const activeButtonId: Writable<string | null> = writable(null);
  
    // Function to update the active button ID
    function setActiveButtonId(id: string) {
      activeButtonId.set(id);
    }
  
    // Provide a context with the activeButtonId store and the setActiveButtonId function
    setContext('toggleGroup', { activeButtonId, setActiveButtonId });
  
    // Optional: Clear the active button when the group is destroyed
    onDestroy(() => activeButtonId.set(null));
  </script>
  
  <div>
    <slot></slot>
  </div>