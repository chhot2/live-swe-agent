<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { HistoryEntry } from '../../lib/types';

  // Props
  let {
    history = $bindable<HistoryEntry[]>([]),
    onNavigate,
    onClose
  }: {
    history: HistoryEntry[];
    onNavigate?: (url: string) => void;
    onClose?: () => void;
  } = $props();

  // Local state
  let searchQuery = $state('');
  let filterDate = $state<'today' | 'week' | 'month' | 'all'>('all');

  // Derived state
  let filteredHistory = $derived(() => {
    let result = history;
    
    // Filter by date
    if (filterDate !== 'all') {
      const now = new Date();
      const cutoff = new Date();
      
      switch (filterDate) {
        case 'today':
          cutoff.setHours(0, 0, 0, 0);
          break;
        case 'week':
          cutoff.setDate(now.getDate() - 7);
          break;
        case 'month':
          cutoff.setMonth(now.getMonth() - 1);
          break;
      }
      
      result = result.filter(h => {
        const visitDate = new Date(h.visited_at || 0);
        return visitDate >= cutoff;
      });
    }
    
    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      result = result.filter(h => 
        h.title?.toLowerCase().includes(query) ||
        h.url?.toLowerCase().includes(query)
      );
    }
    
    return result;
  });

  // Group history by date
  let groupedHistory = $derived(() => {
    const groups: Record<string, HistoryEntry[]> = {};
    
    for (const entry of filteredHistory()) {
      const date = new Date(entry.visited_at || 0);
      const dateKey = date.toLocaleDateString('en-US', { 
        weekday: 'long', 
        year: 'numeric', 
        month: 'long', 
        day: 'numeric' 
      });
      
      if (!groups[dateKey]) {
        groups[dateKey] = [];
      }
      groups[dateKey].push(entry);
    }
    
    return groups;
  });

  function navigateToEntry(entry: HistoryEntry) {
    if (onNavigate && entry.url) {
      onNavigate(entry.url);
    }
  }

  async function deleteEntry(entry: HistoryEntry) {
    try {
      await invoke('delete_history_entry', { id: entry.id });
      history = history.filter(h => h.id !== entry.id);
    } catch (e) {
      console.error('Failed to delete history entry:', e);
    }
  }

  async function clearAllHistory() {
    if (!confirm('Are you sure you want to clear all browsing history?')) {
      return;
    }
    
    try {
      await invoke('clear_history');
      history = [];
    } catch (e) {
      console.error('Failed to clear history:', e);
    }
  }

  function formatTime(timestamp: string | number): string {
    const date = new Date(timestamp);
    return date.toLocaleTimeString('en-US', { 
      hour: '2-digit', 
      minute: '2-digit' 
    });
  }

  function getFavicon(url: string): string {
    try {
      const urlObj = new URL(url);
      return `https://www.google.com/s2/favicons?domain=${urlObj.hostname}&sz=32`;
    } catch {
      return '';
    }
  }
</script>

<div class="history-panel">
  <div class="panel-header">
    <h3>History</h3>
    <div class="header-actions">
      <button class="btn-clear" onclick={clearAllHistory}>Clear All</button>
      <button class="close-btn" onclick={onClose}>×</button>
    </div>
  </div>
  
  <div class="filters">
    <div class="search-box">
      <input 
        type="text" 
        placeholder="Search history..." 
        bind:value={searchQuery}
      />
    </div>
    
    <div class="date-filters">
      <button 
        class="filter-btn" 
        class:active={filterDate === 'all'} 
        onclick={() => filterDate = 'all'}
      >
        All
      </button>
      <button 
        class="filter-btn" 
        class:active={filterDate === 'today'} 
        onclick={() => filterDate = 'today'}
      >
        Today
      </button>
      <button 
        class="filter-btn" 
        class:active={filterDate === 'week'} 
        onclick={() => filterDate = 'week'}
      >
        This Week
      </button>
      <button 
        class="filter-btn" 
        class:active={filterDate === 'month'} 
        onclick={() => filterDate = 'month'}
      >
        This Month
      </button>
    </div>
  </div>
  
  <div class="history-list">
    {#each Object.entries(groupedHistory()) as [date, entries] (date)}
      <div class="date-group">
        <div class="date-header">{date}</div>
        {#each entries as entry (entry.id)}
          <div class="history-item">
            <div class="history-content" onclick={() => navigateToEntry(entry)}>
              <img class="history-favicon" src={getFavicon(entry.url || '')} alt="" />
              <div class="history-info">
                <span class="history-title">{entry.title || 'Untitled'}</span>
                <span class="history-url">{entry.url}</span>
              </div>
              <span class="history-time">{formatTime(entry.visited_at || 0)}</span>
            </div>
            <button 
              class="btn-delete" 
              onclick={() => deleteEntry(entry)} 
              title="Delete"
            >
              🗑️
            </button>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        {#if searchQuery}
          No history matches your search.
        {:else}
          No browsing history yet.
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .history-panel {
    background: #1a1a2e;
    border-radius: 8px;
    overflow: hidden;
    max-height: 500px;
    display: flex;
    flex-direction: column;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid #2a2a45;
  }

  .panel-header h3 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: #e0e0e0;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .btn-clear {
    padding: 4px 10px;
    background: #ef4444;
    border: none;
    border-radius: 4px;
    color: white;
    font-size: 11px;
    cursor: pointer;
  }

  .btn-clear:hover {
    background: #dc2626;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: #808090;
    font-size: 20px;
    cursor: pointer;
    padding: 0;
    line-height: 1;
  }

  .close-btn:hover {
    color: #e0e0e0;
  }

  .filters {
    padding: 12px 16px;
    border-bottom: 1px solid #2a2a45;
  }

  .search-box {
    margin-bottom: 12px;
  }

  .search-box input {
    width: 100%;
    padding: 8px 12px;
    background: #0d0d1a;
    border: 1px solid #2a2a45;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 13px;
  }

  .search-box input:focus {
    outline: none;
    border-color: #4f46e5;
  }

  .date-filters {
    display: flex;
    gap: 8px;
  }

  .filter-btn {
    padding: 6px 12px;
    background: #2a2a45;
    border: 1px solid #3a3a55;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 11px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .filter-btn:hover, .filter-btn.active {
    background: #3a3a55;
    border-color: #4a4a65;
  }

  .history-list {
    flex: 1;
    overflow-y: auto;
  }

  .date-group {
    border-bottom: 1px solid #2a2a45;
  }

  .date-header {
    padding: 8px 16px;
    background: #0d0d1a;
    font-size: 12px;
    font-weight: 600;
    color: #808090;
    position: sticky;
    top: 0;
  }

  .history-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    transition: background 0.2s;
  }

  .history-item:hover {
    background: #2a2a45;
  }

  .history-content {
    display: flex;
    align-items: center;
    gap: 12px;
    flex: 1;
    cursor: pointer;
    overflow: hidden;
  }

  .history-favicon {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .history-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
    flex: 1;
  }

  .history-title {
    font-size: 13px;
    color: #e0e0e0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .history-url {
    font-size: 11px;
    color: #808090;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .history-time {
    font-size: 11px;
    color: #808090;
    flex-shrink: 0;
    margin-left: 12px;
  }

  .btn-delete {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    font-size: 12px;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .history-item:hover .btn-delete {
    opacity: 1;
  }

  .btn-delete:hover {
    background: #ef4444;
  }

  .empty-state {
    padding: 20px;
    text-align: center;
    color: #808090;
    font-size: 13px;
  }
</style>
