<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  let error: Error | null = null;
  let errorId: string = '';
  const dispatch = createEventDispatcher();
  
  // Development mode detection for Tauri
  const isDevelopment = () => {
    return typeof window !== 'undefined' && 
           (window.location.hostname === 'localhost' ||
            window.location.hostname === '127.0.0.1');
  };
  
  // Catch errors in child components
  export let fallback: boolean = true;
  
  function handleError(err: Error) {
    error = err;
    errorId = Math.random().toString(36).substr(2, 9);
    dispatch('error', { error: err, id: errorId });
  }
  
  function handleRetry() {
    error = null;
    dispatch('retry');
  }
  
  // Error boundary wrapper
  function wrapSlot() {
    try {
      // This will be replaced by Svelte's error handling
      return $$slots.default;
    } catch (err) {
      handleError(err as Error);
    }
  }
</script>

{#if error}
  <div class="error-boundary">
    <div class="error-icon">⚠️</div>
    <div class="error-content">
      <h3>Something went wrong</h3>
      <p class="error-message">{error.message}</p>
      {#if isDevelopment()}
        <details class="error-details">
          <summary>Error Details</summary>
          <pre>{error.stack}</pre>
        </details>
      {/if}
      <div class="error-actions">
        <button class="btn-retry" on:click={handleRetry}>Try Again</button>
        <button class="btn-report" on:click={() => dispatch('report', { error, id: errorId })}>
          Report Issue
        </button>
      </div>
    </div>
  </div>
{:else}
  <slot />
{/if}

<style>
  .error-boundary {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 20px;
    background: #3a1a2c;
    border: 1px solid #ff5c8a;
    border-radius: 8px;
    color: #ffb3c8;
    margin: 10px;
  }
  
  .error-icon {
    font-size: 24px;
    flex-shrink: 0;
  }
  
  .error-content {
    flex: 1;
  }
  
  .error-content h3 {
    margin: 0 0 8px;
    font-size: 16px;
    color: #ffc8dc;
  }
  
  .error-message {
    margin: 0 0 12px;
    font-size: 14px;
    opacity: 0.9;
  }
  
  .error-details {
    margin-bottom: 12px;
  }
  
  .error-details summary {
    cursor: pointer;
    font-size: 12px;
    opacity: 0.8;
    margin-bottom: 8px;
  }
  
  .error-details pre {
    background: #2a0f1f;
    padding: 8px;
    border-radius: 4px;
    font-size: 11px;
    overflow-x: auto;
    white-space: pre-wrap;
  }
  
  .error-actions {
    display: flex;
    gap: 8px;
  }
  
  .btn-retry, .btn-report {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
    transition: opacity 0.2s;
  }
  
  .btn-retry {
    background: #ff5c8a;
    color: white;
  }
  
  .btn-retry:hover {
    background: #ff4477;
  }
  
  .btn-report {
    background: transparent;
    color: #ffb3c8;
    border: 1px solid #ff5c8a;
  }
  
  .btn-report:hover {
    background: #ff5c8a20;
  }
</style>
