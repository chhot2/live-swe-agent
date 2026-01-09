<script lang="ts">
  export let width: string = '100%';
  export let height: string = '20px';
  export let rounded: boolean = true;
  export let count: number = 1;
</script>

<div class="skeleton-container">
  {#each Array(count) as _}
    <div 
      class="skeleton" 
      class:rounded
      style="width: {width}; height: {height};"
    ></div>
  {/each}
</div>

<style>
  .skeleton-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  .skeleton {
    background: linear-gradient(90deg, #1f2a45 25%, #2a3750 50%, #1f2a45 75%);
    background-size: 200% 100%;
    animation: loading 1.5s infinite;
  }
  
  .skeleton.rounded {
    border-radius: 4px;
  }
  
  @keyframes loading {
    0% {
      background-position: 200% 0;
    }
    100% {
      background-position: -200% 0;
    }
  }
</style>
