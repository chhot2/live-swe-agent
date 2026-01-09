<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import type { BrowserState } from '../../lib/types';
  import { createDebounce, isValidUrl } from '../../lib/utils';
  
  export let browserState: BrowserState | null = null;
  export let url: string = '';
  
  const dispatch = createEventDispatcher();
  
  function handleFocusUrl() {
    if (urlInput) {
      urlInput.focus();
      urlInput.select();
    }
  }
  let urlInput: HTMLInputElement;
  let suggestions: string[] = [];
  let showSuggestions = false;
  
  // Debounced navigation suggestion fetch
  const debouncedFetchSuggestions = createDebounce(async (query: string) => {
    if (query.length < 2) {
      suggestions = [];
      return;
    }
    
    // Simple suggestion logic - can be enhanced with actual API
    if (isValidUrl(query)) {
      suggestions = [];
    } else {
      suggestions = [
        `https://www.google.com/search?q=${encodeURIComponent(query)}`,
        `https://${query}.com`,
        `https://www.${query}.com`
      ].filter(u => u.length < 100);
    }
    showSuggestions = suggestions.length > 0;
  }, 300);
  
  function handleNavigate() {
    let navigateUrl = url.trim();
    if (!navigateUrl) return;
    
    // Add protocol if missing
    if (!navigateUrl.startsWith('http://') && !navigateUrl.startsWith('https://')) {
      // Check if it looks like a URL
      if (navigateUrl.includes('.') && !navigateUrl.includes(' ')) {
        navigateUrl = 'https://' + navigateUrl;
      } else {
        // Treat as search query
        navigateUrl = `https://www.google.com/search?q=${encodeURIComponent(navigateUrl)}`;
      }
    }
    
    showSuggestions = false;
    dispatch('navigate', { url: navigateUrl });
  }
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      handleNavigate();
    } else if (e.key === 'Escape') {
      showSuggestions = false;
    }
  }
  
  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    url = target.value;
    debouncedFetchSuggestions(url);
  }
  
  function selectSuggestion(suggestion: string) {
    url = suggestion;
    showSuggestions = false;
    urlInput.focus();
    handleNavigate();
  }
  
  function handleBack() {
    dispatch('back');
  }
  
  function handleForward() {
    dispatch('forward');
  }
  
  function handleReload() {
    dispatch('reload');
  }
  
  function handleHome() {
    url = 'https://www.google.com';
    handleNavigate();
  }
  
  function handleBookmark() {
    dispatch('bookmark', { url: browserState?.current_url || url, title: browserState?.title || 'Untitled' });
  }
  
  onMount(() => {
    document.addEventListener('focusUrl', handleFocusUrl);
  });
  
  onDestroy(() => {
    document.removeEventListener('focusUrl', handleFocusUrl);
  });
</script>

<div class="navigation-bar">
  <div class="nav-buttons">
    <button 
      class="nav-btn" 
      on:click={handleBack} 
      disabled={!browserState?.can_go_back}
      title="Go Back"
    >
      ←
    </button>
    <button 
      class="nav-btn" 
      on:click={handleForward} 
      disabled={!browserState?.can_go_forward}
      title="Go Forward"
    >
      →
    </button>
    <button 
      class="nav-btn" 
      on:click={handleReload}
      title="Reload"
    >
      {#if browserState?.is_loading}
        ✕
      {:else}
        ↻
      {/if}
    </button>
    <button class="nav-btn" on:click={handleHome} title="Home">
      ⌂
    </button>
  </div>
  
  <div class="url-bar">
    <div class="url-input-wrapper">
      <input
        type="text"
        bind:value={url}
        on:keydown={handleKeydown}
        on:input={handleInput}
        on:focus={() => showSuggestions = suggestions.length > 0}
        on:blur={() => setTimeout(() => showSuggestions = false, 200)}
        placeholder="Enter URL or search..."
        class="url-input"
        bind:this={urlInput}
      />
      {#if showSuggestions && suggestions.length > 0}
        <div class="suggestions">
          {#each suggestions as suggestion}
            <div 
              class="suggestion-item" 
              on:click={() => selectSuggestion(suggestion)}
              on:keydown={(e) => e.key === 'Enter' && selectSuggestion(suggestion)}
              role="button"
              tabindex="0"
            >
              {suggestion}
            </div>
          {/each}
        </div>
      {/if}
    </div>
    <button class="go-btn" on:click={handleNavigate}>Go</button>
  </div>
  
  <div class="extra-buttons">
    <button class="nav-btn" on:click={handleBookmark} title="Bookmark this page">
      ☆
    </button>
  </div>
</div>

<style>
  .navigation-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: #0c1120;
    border-bottom: 1px solid #1f2a45;
  }
  
  .nav-buttons, .extra-buttons {
    display: flex;
    gap: 4px;
  }
  
  .nav-btn {
    width: 32px;
    height: 32px;
    border: 1px solid #1f2a45;
    background: #151d2e;
    color: #9fb0ce;
    border-radius: 6px;
    cursor: pointer;
    font-size: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }
  
  .nav-btn:hover:not(:disabled) {
    background: #1f2a45;
    color: #e0e7f5;
  }
  
  .nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  
  .url-bar {
    flex: 1;
    display: flex;
    gap: 8px;
    position: relative;
  }
  
  .url-input-wrapper {
    flex: 1;
    position: relative;
  }
  
  .url-input {
    width: 100%;
    background: #151d2e;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    padding: 8px 12px;
    color: #e0e7f5;
    font-size: 13px;
  }
  
  .url-input:focus {
    outline: none;
    border-color: #3b82f6;
  }
  
  .suggestions {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: #151d2e;
    border: 1px solid #1f2a45;
    border-top: none;
    border-radius: 0 0 6px 6px;
    max-height: 200px;
    overflow-y: auto;
    z-index: 1000;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
  }
  
  .suggestion-item {
    padding: 8px 12px;
    color: #e0e7f5;
    font-size: 13px;
    cursor: pointer;
    border-bottom: 1px solid #1f2a45;
    transition: background-color 0.2s;
  }
  
  .suggestion-item:hover {
    background: #1f2a45;
  }
  
  .suggestion-item:last-child {
    border-bottom: none;
  }
  
  .go-btn {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
  }
  
  .go-btn:hover {
    background: #2563eb;
  }
  
  .loading-spinner {
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
