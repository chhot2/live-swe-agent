<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { createEventDispatcher } from 'svelte';
  import type { WebviewTab } from '../../lib/types';
  import {
    fetchWebviewTabs,
    createWebviewTab,
    navigateWebviewTab,
    closeWebviewTab,
    focusWebviewTab
  } from '../../lib/api';
  
  export let selectedTab: WebviewTab | null = null;
  
  const dispatch = createEventDispatcher();
  let tabs: WebviewTab[] = [];
  let loading = false;
  
  // Load tabs on mount
  onMount(async () => {
    await loadTabs();
    
    // Listen for tab updates from backend
    const unlisten = await window.__TAURI__.event.listen('webview-tab-updated', (event) => {
      const updatedTab = event.payload as WebviewTab;
      const index = tabs.findIndex(t => t.tab_id === updatedTab.tab_id);
      if (index !== -1) {
        tabs[index] = updatedTab;
        tabs = [...tabs];
      }
    });
    
    onDestroy(unlisten);
  });
  
  async function loadTabs() {
    loading = true;
    try {
      tabs = await fetchWebviewTabs();
      if (tabs.length > 0 && !selectedTab) {
        selectedTab = tabs[0];
      }
    } catch (error) {
      console.error('Failed to load webview tabs:', error);
    } finally {
      loading = false;
    }
  }
  
  async function handleCreateNew() {
    try {
      const newTab = await createWebviewTab('https://www.google.com');
      tabs = [...tabs, newTab];
      selectedTab = newTab;
      dispatch('tabCreated', { tab: newTab });
    } catch (error) {
      console.error('Failed to create webview tab:', error);
    }
  }
  
  async function handleSelect(tab: WebviewTab) {
    selectedTab = tab;
    try {
      await focusWebviewTab(tab.tab_id);
      dispatch('tabSelected', { tab });
    } catch (error) {
      console.error('Failed to focus webview tab:', error);
    }
  }
  
  async function handleCloseTab(e: MouseEvent, tab: WebviewTab) {
    e.stopPropagation();
    try {
      await closeWebviewTab(tab.tab_id);
      tabs = tabs.filter(t => t.tab_id !== tab.tab_id);
      if (selectedTab?.tab_id === tab.tab_id) {
        selectedTab = tabs.length > 0 ? tabs[0] : null;
      }
      dispatch('tabClosed', { tabId: tab.tab_id });
    } catch (error) {
      console.error('Failed to close webview tab:', error);
    }
  }
  
  async function handleNavigate(url: string) {
    if (!selectedTab) return;
    
    try {
      await navigateWebviewTab(selectedTab.tab_id, url);
      // Update local state immediately for responsiveness
      const index = tabs.findIndex(t => t.tab_id === selectedTab.tab_id);
      if (index !== -1) {
        tabs[index].url = url;
        tabs[index].is_loading = true;
        tabs = [...tabs];
      }
      dispatch('navigated', { tabId: selectedTab.tab_id, url });
    } catch (error) {
      console.error('Failed to navigate webview tab:', error);
    }
  }
  
  // Expose navigation handler to parent
  export async function navigate(url: string) {
    await handleNavigate(url);
  }
  
  // Expose tab creation handler
  export async function createTab(url?: string) {
    try {
      const newTab = await createWebviewTab(url);
      tabs = [...tabs, newTab];
      selectedTab = newTab;
      dispatch('tabCreated', { tab: newTab });
      return newTab;
    } catch (error) {
      console.error('Failed to create webview tab:', error);
      throw error;
    }
  }
</script>

<div class="webview-browser">
  <div class="tab-strip">
    <div class="tabs-scroll">
      {#each tabs as tab (tab.tab_id)}
        <button 
          class="tab"
          class:selected={selectedTab?.tab_id === tab.tab_id}
          class:loading={tab.is_loading}
          on:click={() => handleSelect(tab)}
          title="{tab.title} - {tab.url}"
        >
          <span class="tab-title">
            {#if tab.is_loading}
              <div class="tab-spinner">⟳</div>
            {:else}
              {tab.title || 'New Tab'}
            {/if}
          </span>
          <button 
            class="tab-close"
            on:click={(e) => handleCloseTab(e, tab)}
            title="Close tab"
          >×</button>
        </button>
      {/each}
    </div>
    <button class="new-tab-btn" on:click={handleCreateNew} title="New Tab">+</button>
  </div>
  
  <div class="browser-info">
    {#if selectedTab}
      <div class="info-item">
        <span class="info-label">URL:</span>
        <span class="info-value">{selectedTab.url}</span>
      </div>
      <div class="info-item">
        <span class="info-label">Title:</span>
        <span class="info-value">{selectedTab.title || 'Loading...'}</span>
      </div>
      <div class="info-item">
        <span class="info-label">Status:</span>
        <span class="info-value">
          {#if selectedTab.is_loading}
            Loading...
          {:else}
            Ready
          {/if}
        </span>
      </div>
    {:else}
      <div class="no-tabs">
        <p>No browser tabs open. Click + to create a new tab.</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .webview-browser {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: #080c18;
  }
  
  .tab-strip {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: #0c1120;
    border-bottom: 1px solid #1f2a45;
    min-height: 44px;
  }
  
  .tabs-scroll {
    display: flex;
    gap: 4px;
    flex: 1;
    overflow-x: auto;
    scrollbar-width: thin;
  }
  
  .tabs-scroll::-webkit-scrollbar {
    height: 4px;
  }
  
  .tabs-scroll::-webkit-scrollbar-track {
    background: transparent;
  }
  
  .tabs-scroll::-webkit-scrollbar-thumb {
    background: #1f2a45;
    border-radius: 2px;
  }
  
  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: #151d2e;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    color: #9fb0ce;
    cursor: pointer;
    font-size: 12px;
    min-width: 120px;
    max-width: 200px;
    transition: all 0.2s;
  }
  
  .tab:hover {
    background: #1c2944;
    color: #e0e7f5;
  }
  
  .tab.selected {
    background: #3b82f6;
    color: white;
    border-color: #3b82f6;
  }
  
  .tab.loading {
    opacity: 0.8;
  }
  
  .tab-title {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    align-items: center;
    gap: 4px;
  }
  
  .tab-spinner {
    animation: spin 1s linear infinite;
    font-size: 10px;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  .tab-close {
    background: transparent;
    border: none;
    color: #6b7a9a;
    font-size: 14px;
    cursor: pointer;
    padding: 0 2px;
    line-height: 1;
    border-radius: 3px;
    flex-shrink: 0;
  }
  
  .tab-close:hover {
    background: #ff5c8a33;
    color: #ff5c8a;
  }
  
  .tab.selected .tab-close {
    color: rgba(255, 255, 255, 0.8);
  }
  
  .tab.selected .tab-close:hover {
    background: rgba(255, 255, 255, 0.2);
    color: white;
  }
  
  .new-tab-btn {
    width: 28px;
    height: 28px;
    background: #151d2e;
    border: 1px dashed #1f2a45;
    border-radius: 6px;
    color: #6b7a9a;
    font-size: 18px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: all 0.2s;
  }
  
  .new-tab-btn:hover {
    background: #1c2944;
    border-style: solid;
    color: #9fb0ce;
  }
  
  .browser-info {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
  }
  
  .info-item {
    display: flex;
    gap: 10px;
    margin-bottom: 12px;
    font-size: 13px;
  }
  
  .info-label {
    color: #6b7a9a;
    min-width: 60px;
  }
  
  .info-value {
    color: #e0e7f5;
    word-break: break-all;
  }
  
  .no-tabs {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #6b7a9a;
    font-size: 14px;
  }
</style>
