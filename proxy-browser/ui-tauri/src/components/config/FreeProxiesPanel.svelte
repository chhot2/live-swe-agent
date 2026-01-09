<script lang="ts">
  import { onMount } from 'svelte';
  import type { FreeProxy, ProxyTestResult } from '../../lib/types';
  import { fetchFreeProxies, getFreeProxies, testProxy, setActiveProxy, clearFreeProxies, removeDeadProxies } from '../../lib/api';
  
  let proxies: FreeProxy[] = [];
  let loading = false;
  let testing: string | null = null;
  let error: string | null = null;
  
  let filterProtocol = 'all';
  let filterCountry = '';
  
  $: filteredProxies = proxies.filter(p => {
    if (filterProtocol !== 'all' && p.protocol !== filterProtocol) return false;
    if (filterCountry && !p.country.toLowerCase().includes(filterCountry.toLowerCase())) return false;
    return true;
  });
  
  onMount(async () => {
    try {
      proxies = await getFreeProxies();
    } catch (e) {
      // No existing proxies
    }
  });
  
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
  
  async function handleUse(proxy: FreeProxy) {
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
    proxies = proxies.filter(p => p.is_working);
  }
</script>

<div class="config-section">
  <div class="section-header">
    <h3>Free Proxies</h3>
    <span class="count">{proxies.length} proxies</span>
  </div>
  
  <div class="actions-row">
    <button class="btn-secondary" on:click={handleFetch} disabled={loading}>
      {loading ? 'Fetching...' : 'Fetch Proxies'}
    </button>
    <button class="btn-secondary" on:click={handleRemoveDead} disabled={loading}>
      Remove Dead
    </button>
    <button class="btn-secondary danger" on:click={handleClear} disabled={loading}>
      Clear All
    </button>
  </div>
  
  <div class="filters">
    <select bind:value={filterProtocol}>
      <option value="all">All Protocols</option>
      <option value="http">HTTP</option>
      <option value="https">HTTPS</option>
      <option value="socks4">SOCKS4</option>
      <option value="socks5">SOCKS5</option>
    </select>
    <input type="text" bind:value={filterCountry} placeholder="Filter by country..." />
  </div>
  
  <div class="proxy-list">
    {#each filteredProxies.slice(0, 50) as proxy}
      <div class="proxy-item" class:working={proxy.is_working}>
        <div class="proxy-main">
          <span class="proxy-address">{proxy.ip}:{proxy.port}</span>
          <span class="proxy-protocol">{proxy.protocol}</span>
        </div>
        <div class="proxy-meta">
          <span>{proxy.country}</span>
          {#if proxy.speed > 0}
            <span>{proxy.speed}ms</span>
          {/if}
          <span class="provider">{proxy.provider}</span>
        </div>
        <div class="proxy-actions">
          <button 
            class="btn-sm" 
            on:click={() => handleTest(proxy)}
            disabled={testing === `${proxy.ip}:${proxy.port}`}
          >
            {testing === `${proxy.ip}:${proxy.port}` ? '...' : 'Test'}
          </button>
          <button class="btn-sm primary" on:click={() => handleUse(proxy)}>Use</button>
        </div>
      </div>
    {/each}
    
    {#if filteredProxies.length === 0}
      <div class="empty">No proxies found. Click "Fetch Proxies" to get started.</div>
    {/if}
    
    {#if filteredProxies.length > 50}
      <div class="more">+ {filteredProxies.length - 50} more proxies</div>
    {/if}
  </div>
  
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
  
  .count {
    font-size: 12px;
    color: #9fb0ce;
  }
  
  .actions-row {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
  }
  
  .btn-secondary {
    background: #1c2944;
    border: 1px solid #2b3b60;
    color: #c7d4ec;
    padding: 6px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
  }
  
  .btn-secondary:hover {
    background: #253552;
  }
  
  .btn-secondary.danger:hover {
    background: #3a1a2c;
    border-color: #ff5c8a;
  }
  
  .filters {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
  }
  
  .filters select, .filters input {
    flex: 1;
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    padding: 6px 10px;
    color: #e0e7f5;
    font-size: 12px;
  }
  
  .proxy-list {
    max-height: 300px;
    overflow-y: auto;
  }
  
  .proxy-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px;
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    margin-bottom: 6px;
  }
  
  .proxy-item.working {
    border-color: #22c55e;
  }
  
  .proxy-main {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .proxy-address {
    font-family: monospace;
    font-size: 12px;
    color: #e0e7f5;
  }
  
  .proxy-protocol {
    font-size: 10px;
    padding: 2px 6px;
    background: #1c2944;
    border-radius: 4px;
    color: #9fb0ce;
    text-transform: uppercase;
  }
  
  .proxy-meta {
    display: flex;
    gap: 8px;
    font-size: 11px;
    color: #9fb0ce;
  }
  
  .provider {
    color: #6b7a9a;
  }
  
  .proxy-actions {
    display: flex;
    gap: 4px;
  }
  
  .btn-sm {
    background: #1c2944;
    border: 1px solid #2b3b60;
    color: #c7d4ec;
    padding: 4px 8px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 11px;
  }
  
  .btn-sm.primary {
    background: #3b82f6;
    border-color: #3b82f6;
    color: white;
  }
  
  .empty, .more {
    text-align: center;
    padding: 16px;
    color: #6b7a9a;
    font-size: 12px;
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
