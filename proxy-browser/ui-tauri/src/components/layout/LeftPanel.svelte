<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Tab, Country, ValidationResponse } from '../../lib/types';
  import BrowserView from '../browser/BrowserView.svelte';
  import EnhancedTabList from '../browser/EnhancedTabList.svelte';
  import SkeletonLoader from '../ui/SkeletonLoader.svelte';
  import LoadingSpinner from '../ui/LoadingSpinner.svelte';
  import { closeTab } from '../../lib/api';
  import { tabOperations } from '../../lib/stores';
  
  export let tabs: Tab[] = [];
  export let countries: Country[] = [];
  export let selectedTab: Tab | null = null;
  export let validation: ValidationResponse | null = null;
  export let loading = false;
  export let validating = false;
  
  const dispatch = createEventDispatcher();
  
  function handleSelect(tab: Tab) {
    dispatch('select', tab);
  }
  
  function handleCreateNew() {
    dispatch('createRandom');
  }
  
  async function handleCloseTab(e: MouseEvent, tabId: string) {
    e.stopPropagation();
    try {
      await closeTab(tabId);
      dispatch('tabClosed', { tabId });
    } catch (err) {
      console.error('Failed to close tab:', err);
    }
  }
</script>

<div class="left-panel">
  <div class="tab-strip">
    <div class="tabs-header">
      <h3>Tabs ({tabs.length})</h3>
      <button class="new-tab-btn" on:click={handleCreateNew} title="New Tab">+</button>
    </div>
    
    <div class="tabs-container">
      <EnhancedTabList
        {tabs}
        bind:selectedTab
        height={300}
        itemHeight={48}
        on:select={(e) => handleSelect(e.detail.tab)}
      />
    </div>
    
    <div class="status-area">
      {#if loading}
        <div class="status-pill">
          <LoadingSpinner size="small" />
          <span>Loading...</span>
        </div>
      {/if}
      {#if validating}
        <div class="status-pill">
          <LoadingSpinner size="small" />
          <span>Validating...</span>
        </div>
      {/if}
    </div>
  </div>
  
  <div class="browser-container">
    <BrowserView tab={selectedTab} />
  </div>
</div>

<style>
  .left-panel {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: #080c18;
  }
  
  .tab-strip {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #0c1120;
    border-right: 1px solid #1f2a45;
    min-height: 0;
  }
  
  .tabs-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: #0a0e1a;
    border-bottom: 1px solid #1f2a45;
  }
  
  .tabs-header h3 {
    color: #e0e7f5;
    font-size: 14px;
    font-weight: 600;
    margin: 0;
  }
  
  .tabs-container {
    flex: 1;
    min-height: 0;
    padding: 8px;
    background: #0a0e1a;
  }
  
  .tabs-scroll::-webkit-scrollbar {
    height: 4px;
  }
  
  .tabs-scroll::-webkit-scrollbar-thumb {
    background: #1f2a45;
    border-radius: 2px;
  }
  
  .tab-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: #151d2e;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    color: #9fb0ce;
    font-size: 12px;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.2s;
  }
  
  .tab-button:hover {
    background: #1c2944;
  }
  
  .tab-button.active {
    background: #1f2a45;
    border-color: #3b82f6;
    color: #e0e7f5;
  }
  
  .tab-flag {
    font-size: 11px;
    opacity: 0.8;
  }
  
  .tab-ip {
    font-family: monospace;
    font-size: 11px;
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
  }
  
  .tab-close:hover {
    background: #ff5c8a33;
    color: #ff5c8a;
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
  }
  
  .new-tab-btn:hover {
    background: #1c2944;
    border-style: solid;
    color: #9fb0ce;
  }
  
  .status-area {
    display: flex;
    gap: 6px;
    flex-shrink: 0;
  }
  
  .status-pill {
    background: #1c2944;
    border: 1px solid #2b3b60;
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 11px;
    color: #c7d4ec;
    display: flex;
    align-items: center;
    gap: 6px;
  }
  
  .browser-container {
    flex: 1;
    overflow: hidden;
  }
</style>
