<script lang="ts">
  import type { WebviewTab } from '../../lib/types';

  interface BrowserTab extends WebviewTab {
    favicon?: string;
    isPinned?: boolean;
    isMuted?: boolean;
  }

  // Props
  let {
    tabs = $bindable<BrowserTab[]>([]),
    activeTabId = $bindable<string | null>(null),
    onTabSelect,
    onTabClose,
    onNewTab,
    onTabPin,
    onTabMute
  }: {
    tabs: BrowserTab[];
    activeTabId: string | null;
    onTabSelect?: (tabId: string) => void;
    onTabClose?: (tabId: string) => void;
    onNewTab?: () => void;
    onTabPin?: (tabId: string) => void;
    onTabMute?: (tabId: string) => void;
  } = $props();

  function selectTab(tabId: string) {
    activeTabId = tabId;
    if (onTabSelect) onTabSelect(tabId);
  }

  function closeTab(e: Event, tabId: string) {
    e.stopPropagation();
    if (onTabClose) onTabClose(tabId);
  }

  function pinTab(e: Event, tabId: string) {
    e.stopPropagation();
    if (onTabPin) onTabPin(tabId);
  }

  function muteTab(e: Event, tabId: string) {
    e.stopPropagation();
    if (onTabMute) onTabMute(tabId);
  }

  function handleDragStart(e: DragEvent, tabId: string) {
    e.dataTransfer?.setData('text/plain', tabId);
  }

  function handleDrop(e: DragEvent, targetTabId: string) {
    e.preventDefault();
    const sourceTabId = e.dataTransfer?.getData('text/plain');
    if (sourceTabId && sourceTabId !== targetTabId) {
      // Reorder tabs
      const sourceIndex = tabs.findIndex(t => t.tab_id === sourceTabId);
      const targetIndex = tabs.findIndex(t => t.tab_id === targetTabId);
      if (sourceIndex !== -1 && targetIndex !== -1) {
        const [movedTab] = tabs.splice(sourceIndex, 1);
        tabs.splice(targetIndex, 0, movedTab);
        tabs = [...tabs];
      }
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function truncateTitle(title: string | undefined, maxLength = 20): string {
    if (!title) return 'New Tab';
    return title.length > maxLength ? title.substring(0, maxLength) + '...' : title;
  }
</script>

<div class="tab-bar">
  <div class="tabs-container">
    {#each tabs as tab (tab.tab_id)}
      <div
        class="tab"
        class:active={tab.tab_id === activeTabId}
        class:pinned={tab.isPinned}
        onclick={() => selectTab(tab.tab_id)}
        draggable="true"
        ondragstart={(e) => handleDragStart(e, tab.tab_id)}
        ondrop={(e) => handleDrop(e, tab.tab_id)}
        ondragover={handleDragOver}
        role="tab"
        tabindex="0"
        aria-selected={tab.tab_id === activeTabId}
      >
        {#if tab.favicon}
          <img class="tab-favicon" src={tab.favicon} alt="" />
        {:else}
          <span class="tab-favicon-placeholder">🌐</span>
        {/if}
        
        <span class="tab-title" title={tab.title || tab.url}>
          {truncateTitle(tab.title || tab.url)}
        </span>
        
        {#if tab.isMuted}
          <button class="tab-mute" onclick={(e) => muteTab(e, tab.tab_id)} title="Unmute tab">
            🔇
          </button>
        {/if}
        
        {#if !tab.isPinned}
          <button class="tab-close" onclick={(e) => closeTab(e, tab.tab_id)} title="Close tab">
            ×
          </button>
        {/if}
      </div>
    {/each}
  </div>
  
  <button class="new-tab-btn" onclick={onNewTab} title="New Tab (Ctrl+T)">
    +
  </button>
</div>

<style>
  .tab-bar {
    display: flex;
    align-items: center;
    background: #1a1a2e;
    border-bottom: 1px solid #2a2a45;
    padding: 4px 8px;
    min-height: 40px;
  }

  .tabs-container {
    display: flex;
    flex: 1;
    overflow-x: auto;
    gap: 2px;
  }

  .tabs-container::-webkit-scrollbar {
    height: 4px;
  }

  .tabs-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .tabs-container::-webkit-scrollbar-thumb {
    background: #3a3a55;
    border-radius: 2px;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: #0d0d1a;
    border-radius: 8px 8px 0 0;
    cursor: pointer;
    min-width: 120px;
    max-width: 200px;
    transition: all 0.2s;
    border: 1px solid transparent;
    border-bottom: none;
  }

  .tab:hover {
    background: #2a2a45;
  }

  .tab.active {
    background: #2a2a45;
    border-color: #3a3a55;
  }

  .tab.pinned {
    min-width: 40px;
    max-width: 40px;
    padding: 8px;
    justify-content: center;
  }

  .tab.pinned .tab-title {
    display: none;
  }

  .tab-favicon {
    width: 16px;
    height: 16px;
    border-radius: 2px;
    flex-shrink: 0;
  }

  .tab-favicon-placeholder {
    font-size: 14px;
    flex-shrink: 0;
  }

  .tab-title {
    flex: 1;
    font-size: 12px;
    color: #e0e0e0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tab-mute, .tab-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border: none;
    background: transparent;
    color: #808090;
    cursor: pointer;
    border-radius: 4px;
    font-size: 14px;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .tab-mute:hover, .tab-close:hover {
    background: #3a3a55;
    color: #e0e0e0;
  }

  .tab-close:hover {
    background: #ef4444;
    color: white;
  }

  .new-tab-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: transparent;
    color: #808090;
    cursor: pointer;
    border-radius: 6px;
    font-size: 20px;
    transition: all 0.2s;
    margin-left: 4px;
  }

  .new-tab-btn:hover {
    background: #2a2a45;
    color: #e0e0e0;
  }
</style>
