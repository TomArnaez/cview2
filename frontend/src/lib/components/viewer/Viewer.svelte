<script lang="ts">
    import { writable, derived } from 'svelte/store';
  
    let items = writable([]);
  
    function addItem() {
      items.update(currentItems => {
        return [...currentItems, `Item ${currentItems.length + 1}`];
      });
    }
  
    function removeItem(index: number) {
      items.update(currentItems => {
        return currentItems.filter((_, i) => i !== index);
      });
    }
  
    // Derived store to calculate the optimal grid layout
    let gridStyle = derived(items, $items => {
      // Calculate the number of columns based on the item count to attempt a square-ish layout
      let columns = Math.ceil(Math.sqrt($items.length));
      return `grid-template-columns: repeat(${columns}, minmax(150px, 1fr));`;
    });
  </script>
  
  <style>
    .grid {
      display: grid;
      gap: 4px; /* Adjust gap size as needed */
    }
  </style>
  
  <button on:click={addItem}>Add Item</button>
  
  <div class="grid" style={$gridStyle}>
    {#each $items as item, index}
      <div class="bg-blue-500 p-4 text-white">
        {item}
        <button on:click={() => removeItem(index)} class="ml-2 bg-red-500 p-1">Remove</button>
      </div>
    {/each}
  </div>