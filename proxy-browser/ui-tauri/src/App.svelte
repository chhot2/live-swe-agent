<script lang="ts">
  import BrowserShell from './components/browser/BrowserShell.svelte';
</script>

<BrowserShell />

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
  }
  
  :global(*) {
    box-sizing: border-box;
  }
</style>
