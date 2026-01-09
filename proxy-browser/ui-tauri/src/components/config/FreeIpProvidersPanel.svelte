<script lang="ts">
  import { onMount } from 'svelte';
  import type { FreeProxy, ProxyTestResult } from '../../lib/types';
  import { fetchFreeProxies, getFreeProxies, testProxy, setActiveProxy, clearFreeProxies, removeDeadProxies } from '../../lib/api';
  
  let proxies: FreeProxy[] = [];
  let loading = false;
  let testing: string | null = null;
  let error: string | null = null;
  let filter = {
    protocol: 'all',
    country: '',
    onlyWorking: false,
  };
  
  onMount(async () => {
    await loadProxies();
  });
  
  async function loadProxies() {
    try {
      proxies = await getFreeProxies();
    } catch (e) {
      // No proxies yet
    }
  }
  
  async function handleFetch() {
    loading = true;
    error = null;
    try {
      proxies = await fetchFreeProxies();
    } catch (e) {
      error = 'Failed to fetch proxies';
    } finally {
      loading = false;
    }
  }
  
  async function handleTest(proxy: FreeProxy) {
    testing = `${proxy.ip}:${proxy.port}`;
    try {
      const result = await testProxy(proxy);
      proxies = proxies.map(p => 
        p.ip === proxy.ip && p.port === proxy.port 
          ? { ...p, is_working: result.is_working, speed: result.latency_ms || 0 }
          : p
      );
    } catch (e) {
      // Test failed
    } finally {
      testing = null;
    }
  }
  
  async function handleUseProxy(proxy: FreeProxy) {
    try {
      await setActiveProxy(proxy);
    } catch (e) {
      error = 'Failed to set proxy';
    }
  }
  
  async function handleClear() {
    await clearFreeProxies();
    proxies = [];
  }
  
  async function handleRemoveDead() {
    await removeDeadProxies();
    await loadProxies();
  }
  
  $: filteredProxies = proxies.filter(p => {
    if (filter.protocol !== 'all' && p.protocol !== filter.protocol) return false;
    if (filter.country && !p.country.toLowerCase().includes(filter.country.toLowerCase())) return false;
    if (filter.onlyWorking && !p.is_working) return false;
    return true;
  });
</script>

<div class="config-section">
  <div class="section-header">
    <h3>Free IP Providers</h3>
    <button class="btn-primary btn-sm" on:click={handleFetch} disabled={loading}>
      {loading ? 'Fetching...' : '↻ Fetch Proxies'}
    </button>
  </div>
  
  <div class="filters">
    <select bind:value={filter.protocol}>
      <option value="all">All Protocols</option>
      <option value="http">HTTP</option>
      <option value="https">HTTPS</option>
      <option value="socks4">SOCKS4</option>
      <option value="socks5">SOCKS5</option>
    </select>
    <input type="text" bind:value={filter.country} placeholder="Filter by country..." />
    <label class="checkbox-sm">
      <input type="checkbox" bind:checked={filter.onlyWorking} />
      Working only
    </label>
  </div>
  
  <div class="proxy-list">
    {#if filteredProxies.length === 0}
      <div class="empty">No proxies. Click "Fetch Proxies" to load from providers.</div>
    {:else}
      {#each filteredProxies as proxy}
        <div class="proxy-item" class:working={proxy.is_working} class:failed={proxy.is_working === false}>
          <div class="proxy-info">
            <div class="proxy-address">{proxy.ip}:{proxy.port}</div>
            <div class="proxy-meta">
              <span class="tag protocol">{proxy.protocol.toUpperCase()}</span>
              <span class="tag country">{proxy.country_code}</span>
              {#if proxy.speed > 0}
                <span class="tag speed">{proxy.speed}ms</span>
              {/if}
              <span class="tag provider">{proxy.provider}</span>
            </div>
          </div>
          <div class="proxy-actions">
            <button 
              class="btn-icon" 
              on:click={() => handleTest(proxy)} 
              disabled={testing === `${proxy.ip}:${proxy.port}`}
              title="Test"
            >
              {testing === `${proxy.ip}:${proxy.port}` ? '⏳' : '🔍'}
            </button>
            <button class="btn-icon" on:click={() => handleUseProxy(proxy)} title="Use this proxy">
              ✓
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
  
  {#if proxies.length > 0}
    <div class="actions-row">
      <button class="btn-ghost btn-sm" on:click={handleRemoveDead}>Remove Dead</button>
      <button class="btn-ghost btn-sm" on:click={handleClear}>Clear All</button>
      <span class="count">{filteredProxies.length} / {proxies.length} proxies</span>
    </div>
  {/if}
  
  {#if error}
    <div class="error-msg">{error}</div>
  {/if}
</div>

<style>
  .config-section {
    padding: 16px;
    border-bottom: 1px solid #1f2a45;
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }
  
  h3 {
    margin: 0;
    font-size: 14px;
    color: #c7d4ec;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .filters {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
    flex-wrap: wrap;
  }
  
  .filters select, .filters input[type="text"] {
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    padding: 6px 10px;
    color: #e0e7f5;
    font-size: 12px;
    flex: 1;
    min-width: 100px;
  }
  
  .checkbox-sm {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: #9fb0ce;
    white-space: nowrap;
  }
  
  .proxy-list {
    max-height: 300px;
    overflow-y: auto;
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 8px;
  }
  
  .empty {
    padding: 24px;
    text-align: center;
    color: #9fb0ce;
    font-size: 13px;
  }
  
  .proxy-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 12px;
    border-bottom: 1px solid #1a2440;
  }
  
  .proxy-item:last-child {
    border-bottom: none;
  }
  
  .proxy-item.working {
    background: rgba(34, 197, 94, 0.1);
  }
  
  .proxy-item.failed {
    background: rgba(239, 68, 68, 0.1);
  }
  
  .proxy-address {
    font-family: monospace;
    font-size: 13px;
    color: #e0e7f5;
    margin-bottom: 4px;
  }
  
  .proxy-meta {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }
  
  .tag {
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 4px;
    background: #1a2440;
    color: #9fb0ce;
  }
  
  .tag.protocol { background: #1e3a5f; color: #60a5fa; }
  .tag.country { background: #3a1e5f; color: #a78bfa; }
  .tag.speed { background: #1e5f3a; color: #4ade80; }
  .tag.provider { background: #5f3a1e; color: #fbbf24; }
  
  .proxy-actions {
    display: flex;
    gap: 4px;
  }
  
  .btn-icon {
    background: transparent;
    border: 1px solid #1f2a45;
    color: #9fb0ce;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
  }
  
  .btn-icon:hover {
    background: #1f2a45;
  }
  
  .btn-icon:disabled {
    opacity: 0.5;
  }
  
  .btn-primary {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 6px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
  }
  
  .btn-primary:hover { background: #2563eb; }
  .btn-primary:disabled { opacity: 0.6; }
  
  .btn-ghost {
    background: transparent;
    border: 1px solid #1f2a45;
    color: #9fb0ce;
    padding: 6px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
  }
  
  .btn-ghost:hover { background: #1f2a45; }
  
  .btn-sm { padding: 5px 10px; font-size: 11px; }
  
  .actions-row {
    display: flex;
    gap: 8px;
    margin-top: 12px;
    align-items: center;
  }
  
  .count {
    margin-left: auto;
    font-size: 11px;
    color: #9fb0ce;
  }
  
  .error-msg {
    margin-top: 12px;
    padding: 8px 12px;
    background: #3a1a2c;
    border: 1px solid #ff5c8a;
    color: #ffb3c8;
    border-radius: 6px;
    font-size: 12px;
  }
</style>
