<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import type { Tab, BrowserState } from '../../lib/types';
  import NavigationBar from './NavigationBar.svelte';
  import { navigate, goBack, goForward, reloadPage, getBrowserState, addBookmark } from '../../lib/api';
  
  export let tab: Tab | null = null;
  
  let browserState: BrowserState | null = null;
  let url: string = '';
  let iframeRef: HTMLIFrameElement;
  let loading = false;
  let error: string | null = null;
  
  const dispatch = createEventDispatcher();
  
  $: if (tab) {
    loadBrowserState(tab.tab_id);
  }
  
  async function loadBrowserState(tabId: string) {
    try {
      browserState = await getBrowserState(tabId);
      if (browserState) {
        url = browserState.current_url;
      }
    } catch (e) {
      // No state yet
    }
  }
  
  async function handleNavigate(e: CustomEvent<{ url: string }>) {
    if (!tab) return;
    
    loading = true;
    error = null;
    
    try {
      browserState = await navigate(tab.tab_id, e.detail.url);
      url = browserState.current_url;
      
      // Load in iframe
      if (iframeRef) {
        iframeRef.src = browserState.current_url;
      }
    } catch (err) {
      error = 'Navigation failed';
      console.error(err);
    } finally {
      loading = false;
    }
  }
  
  async function handleBack() {
    if (!tab) return;
    
    try {
      const newUrl = await goBack(tab.tab_id);
      if (newUrl) {
        url = newUrl;
        browserState = await getBrowserState(tab.tab_id);
        if (iframeRef) {
          iframeRef.src = newUrl;
        }
      }
    } catch (err) {
      console.error(err);
    }
  }
  
  async function handleForward() {
    if (!tab) return;
    
    try {
      const newUrl = await goForward(tab.tab_id);
      if (newUrl) {
        url = newUrl;
        browserState = await getBrowserState(tab.tab_id);
        if (iframeRef) {
          iframeRef.src = newUrl;
        }
      }
    } catch (err) {
      console.error(err);
    }
  }
  
  async function handleReload() {
    if (!tab) return;
    
    try {
      const reloadUrl = await reloadPage(tab.tab_id);
      if (reloadUrl && iframeRef) {
        iframeRef.src = reloadUrl;
      }
    } catch (err) {
      console.error(err);
    }
  }
  
  async function handleBookmark(e: CustomEvent<{ url: string; title: string }>) {
    try {
      await addBookmark(e.detail.url, e.detail.title);
      dispatch('bookmarkAdded');
    } catch (err) {
      console.error(err);
    }
  }
  
  function handleIframeLoad() {
    loading = false;
  }
</script>

<div class="browser-view">
  <NavigationBar 
    {browserState}
    bind:url
    on:navigate={handleNavigate}
    on:back={handleBack}
    on:forward={handleForward}
    on:reload={handleReload}
    on:bookmark={handleBookmark}
  />
  
  <div class="browser-content">
    {#if !tab}
      <div class="placeholder">
        <div class="placeholder-icon">🌐</div>
        <h3>No Tab Selected</h3>
        <p>Create or select a tab to start browsing</p>
      </div>
    {:else if loading && !browserState?.current_url}
      <div class="placeholder">
        <div class="loading-spinner">↻</div>
        <p>Loading...</p>
      </div>
    {:else}
      {#key tab.tab_id}
        <div class="iframe-container">
          <iframe 
            bind:this={iframeRef}
            src={browserState?.current_url || 'about:blank'}
            title="Browser"
            sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
            on:load={handleIframeLoad}
          />
          
          {#if loading}
            <div class="loading-overlay">
              <div class="loading-spinner">↻</div>
            </div>
          {/if}
        </div>
        
        <div class="browser-info">
          <span class="ip-badge" title="Tab IP">
            🌍 {tab.ip}
          </span>
          <span class="country-badge">
            {tab.country_name}
          </span>
        </div>
      {/key}
    {/if}
    
    {#if error}
      <div class="error-banner">{error}</div>
    {/if}
  </div>
</div>

<style>
  .browser-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #080c18;
  }
  
  .browser-content {
    flex: 1;
    position: relative;
    overflow: hidden;
  }
  
  .placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #6b7a9a;
    text-align: center;
  }
  
  .placeholder-icon {
    font-size: 64px;
    margin-bottom: 16px;
  }
  
  .placeholder h3 {
    margin: 0 0 8px;
    color: #9fb0ce;
    font-size: 18px;
  }
  
  .placeholder p {
    margin: 0;
    font-size: 14px;
  }
  
  .iframe-container {
    position: relative;
    width: 100%;
    height: calc(100% - 32px);
  }
  
  iframe {
    width: 100%;
    height: 100%;
    border: none;
    background: white;
  }
  
  .loading-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(8, 12, 24, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .loading-spinner {
    font-size: 32px;
    color: #3b82f6;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  .browser-info {
    display: flex;
    gap: 8px;
    padding: 6px 12px;
    background: #0c1120;
    border-top: 1px solid #1f2a45;
  }
  
  .ip-badge, .country-badge {
    font-size: 11px;
    padding: 2px 8px;
    background: #151d2e;
    border: 1px solid #1f2a45;
    border-radius: 4px;
    color: #9fb0ce;
  }
  
  .ip-badge {
    font-family: monospace;
  }
  
  .error-banner {
    position: absolute;
    bottom: 40px;
    left: 50%;
    transform: translateX(-50%);
    background: #3a1a2c;
    border: 1px solid #ff5c8a;
    color: #ffb3c8;
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 12px;
  }
</style>
