<script lang="ts">
  import type { Tab } from '../lib/types';
  import { createEventDispatcher } from 'svelte';

  export let tabs: Tab[] = [];
  export let selectedTabId: string | undefined;

  const dispatch = createEventDispatcher<{ select: Tab }>();

  function selectTab(tab: Tab) {
    dispatch('select', tab);
  }
</script>

<div class="list">
  {#if tabs.length === 0}
    <div class="empty">No tabs yet. Create one to begin.</div>
  {:else}
    {#each tabs as tab}
      <button
        class:selected={tab.tab_id === selectedTabId}
        class="item"
        on:click={() => selectTab(tab)}
      >
        <div class="ip">{tab.ip}</div>
        <div class="meta">
          <span class="pill">{tab.country_code}</span>
          <span>{tab.country_name}</span>
          <span class="muted">{tab.city || 'Unknown'}</span>
        </div>
      </button>
    {/each}
  {/if}
</div>

<style>
  .list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .empty {
    padding: 12px;
    color: #9fb0ce;
    background: #10182c;
    border: 1px dashed #263655;
    border-radius: 10px;
  }
  .item {
    width: 100%;
    text-align: left;
    background: #0f1729;
    border: 1px solid #1f2e4a;
    color: #e8ecf3;
    border-radius: 12px;
    padding: 10px 12px;
    cursor: pointer;
    transition: border 0.15s ease, transform 0.1s ease;
  }
  .item:hover {
    border-color: #3b82f6;
    transform: translateY(-1px);
  }
  .item.selected {
    border-color: #3b82f6;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2);
  }
  .ip {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 14px;
    margin-bottom: 4px;
  }
  .meta {
    display: flex;
    gap: 8px;
    align-items: center;
    font-size: 13px;
    color: #b8c8e6;
    flex-wrap: wrap;
  }
  .pill {
    padding: 2px 8px;
    border-radius: 999px;
    background: #1f2e4a;
    border: 1px solid #2d3d5c;
    color: #cde3ff;
    font-size: 12px;
  }
  .muted {
    color: #7f8fb1;
  }
</style>
