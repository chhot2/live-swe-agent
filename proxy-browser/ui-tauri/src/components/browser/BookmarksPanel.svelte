<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Bookmark } from '../../lib/types';

  // Props
  let {
    bookmarks = $bindable<Bookmark[]>([]),
    onNavigate,
    onClose
  }: {
    bookmarks: Bookmark[];
    onNavigate?: (url: string) => void;
    onClose?: () => void;
  } = $props();

  // Local state
  let searchQuery = $state('');
  let editingBookmark = $state<Bookmark | null>(null);
  let newBookmarkTitle = $state('');
  let newBookmarkUrl = $state('');

  // Derived state
  let filteredBookmarks = $derived(() => {
    if (!searchQuery) return bookmarks;
    const query = searchQuery.toLowerCase();
    return bookmarks.filter(b => 
      b.title?.toLowerCase().includes(query) ||
      b.url?.toLowerCase().includes(query)
    );
  });

  function navigateToBookmark(bookmark: Bookmark) {
    if (onNavigate && bookmark.url) {
      onNavigate(bookmark.url);
    }
  }

  async function deleteBookmark(bookmark: Bookmark) {
    try {
      await invoke('delete_bookmark', { id: bookmark.id });
      bookmarks = bookmarks.filter(b => b.id !== bookmark.id);
    } catch (e) {
      console.error('Failed to delete bookmark:', e);
    }
  }

  function startEdit(bookmark: Bookmark) {
    editingBookmark = bookmark;
    newBookmarkTitle = bookmark.title || '';
    newBookmarkUrl = bookmark.url || '';
  }

  async function saveEdit() {
    if (!editingBookmark) return;
    
    try {
      await invoke('update_bookmark', {
        id: editingBookmark.id,
        title: newBookmarkTitle,
        url: newBookmarkUrl
      });
      
      const index = bookmarks.findIndex(b => b.id === editingBookmark!.id);
      if (index !== -1) {
        bookmarks[index] = {
          ...bookmarks[index],
          title: newBookmarkTitle,
          url: newBookmarkUrl
        };
        bookmarks = [...bookmarks];
      }
      
      editingBookmark = null;
    } catch (e) {
      console.error('Failed to update bookmark:', e);
    }
  }

  function cancelEdit() {
    editingBookmark = null;
    newBookmarkTitle = '';
    newBookmarkUrl = '';
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

<div class="bookmarks-panel">
  <div class="panel-header">
    <h3>Bookmarks</h3>
    <button class="close-btn" onclick={onClose}>×</button>
  </div>
  
  <div class="search-box">
    <input 
      type="text" 
      placeholder="Search bookmarks..." 
      bind:value={searchQuery}
    />
  </div>
  
  <div class="bookmarks-list">
    {#each filteredBookmarks() as bookmark (bookmark.id)}
      <div class="bookmark-item">
        {#if editingBookmark?.id === bookmark.id}
          <div class="edit-form">
            <input 
              type="text" 
              placeholder="Title" 
              bind:value={newBookmarkTitle}
            />
            <input 
              type="text" 
              placeholder="URL" 
              bind:value={newBookmarkUrl}
            />
            <div class="edit-actions">
              <button class="btn-save" onclick={saveEdit}>Save</button>
              <button class="btn-cancel" onclick={cancelEdit}>Cancel</button>
            </div>
          </div>
        {:else}
          <div class="bookmark-content" onclick={() => navigateToBookmark(bookmark)}>
            <img class="bookmark-favicon" src={getFavicon(bookmark.url || '')} alt="" />
            <div class="bookmark-info">
              <span class="bookmark-title">{bookmark.title || 'Untitled'}</span>
              <span class="bookmark-url">{bookmark.url}</span>
            </div>
          </div>
          <div class="bookmark-actions">
            <button class="btn-edit" onclick={() => startEdit(bookmark)} title="Edit">
              ✏️
            </button>
            <button class="btn-delete" onclick={() => deleteBookmark(bookmark)} title="Delete">
              🗑️
            </button>
          </div>
        {/if}
      </div>
    {:else}
      <div class="empty-state">
        {#if searchQuery}
          No bookmarks match your search.
        {:else}
          No bookmarks yet. Add some!
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .bookmarks-panel {
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

  .search-box {
    padding: 12px 16px;
    border-bottom: 1px solid #2a2a45;
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

  .bookmarks-list {
    flex: 1;
    overflow-y: auto;
  }

  .bookmark-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    border-bottom: 1px solid #2a2a45;
    transition: background 0.2s;
  }

  .bookmark-item:hover {
    background: #2a2a45;
  }

  .bookmark-content {
    display: flex;
    align-items: center;
    gap: 12px;
    flex: 1;
    cursor: pointer;
    overflow: hidden;
  }

  .bookmark-favicon {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .bookmark-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
  }

  .bookmark-title {
    font-size: 13px;
    color: #e0e0e0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .bookmark-url {
    font-size: 11px;
    color: #808090;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .bookmark-actions {
    display: flex;
    gap: 4px;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .bookmark-item:hover .bookmark-actions {
    opacity: 1;
  }

  .btn-edit, .btn-delete {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    font-size: 12px;
  }

  .btn-edit:hover {
    background: #3a3a55;
  }

  .btn-delete:hover {
    background: #ef4444;
  }

  .edit-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
  }

  .edit-form input {
    padding: 6px 10px;
    background: #0d0d1a;
    border: 1px solid #2a2a45;
    border-radius: 4px;
    color: #e0e0e0;
    font-size: 12px;
  }

  .edit-actions {
    display: flex;
    gap: 8px;
  }

  .btn-save, .btn-cancel {
    padding: 4px 12px;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
  }

  .btn-save {
    background: #4f46e5;
    color: white;
  }

  .btn-cancel {
    background: #3a3a55;
    color: #e0e0e0;
  }

  .empty-state {
    padding: 20px;
    text-align: center;
    color: #808090;
    font-size: 13px;
  }
</style>
