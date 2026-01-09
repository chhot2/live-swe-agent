<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { FreeProxy, ProxySessionStats } from '../../lib/types';

  // Props using Svelte 5 runes
  let {
    proxies = $bindable<FreeProxy[]>([]),
    currentProxy = $bindable<FreeProxy | null>(null),
    proxyStats = $bindable<ProxySessionStats | null>(null),
    rotationStrategy = $bindable('round_robin'),
    fetchingProxies = $bindable(false),
    validatingProxy = $bindable(false),
    onProxySelect,
    onRefresh
  }: {
    proxies: FreeProxy[];
    currentProxy: FreeProxy | null;
    proxyStats: ProxySessionStats | null;
    rotationStrategy: string;
    fetchingProxies: boolean;
    validatingProxy: boolean;
    onProxySelect?: (proxy: FreeProxy) => void;
    onRefresh?: () => void;
  } = $props();

  // Local state
  let proxyFilter = $state('all');

  // Derived state
  let filteredProxies = $derived(() => {
    if (proxyFilter === 'all') return proxies;
    return proxies.filter(p => p.protocol?.toLowerCase() === proxyFilter);
  });

  async function rotateProxy() {
    try {
      await invoke('rotate_proxy', { strategy: rotationStrategy });
      if (onRefresh) onRefresh();
    } catch (e) {
      console.error('Failed to rotate proxy:', e);
    }
  }

  async function fetchFromProvider(provider: string) {
    fetchingProxies = true;
    try {
      await invoke('fetch_proxies_from_provider', { provider });
      if (onRefresh) onRefresh();
    } catch (e) {
      console.error(`Failed to fetch from ${provider}:`, e);
    } finally {
      fetchingProxies = false;
    }
  }

  async function validateProxy(proxy: FreeProxy) {
    validatingProxy = true;
    try {
      await invoke('validate_proxy', { proxy });
      if (onRefresh) onRefresh();
    } catch (e) {
      console.error('Failed to validate proxy:', e);
    } finally {
      validatingProxy = false;
    }
  }

  function selectProxy(proxy: FreeProxy) {
    if (onProxySelect) onProxySelect(proxy);
  }

  function formatSpeed(speed: number | undefined): string {
    if (!speed) return 'N/A';
    return `${speed}ms`;
  }
</script>

<div class="proxy-panel">
  <!-- Rotation Strategy -->
  <div class="rotation-section">
    <h4>Rotation Strategy</h4>
    <select class="strategy-select" bind:value={rotationStrategy}>
      <option value="round_robin">Round Robin</option>
      <option value="random">Random</option>
      <option value="fastest">Fastest</option>
      <option value="least_used">Least Used</option>
    </select>
    <button class="btn-rotate" onclick={rotateProxy} disabled={fetchingProxies}>
      {fetchingProxies ? 'Rotating...' : 'Rotate Proxy'}
    </button>
  </div>

  <!-- Proxy Stats -->
  {#if proxyStats}
    <div class="proxy-stats">
      <div class="stat-item">
        <span class="label">Total Requests</span>
        <span class="value">{proxyStats.total_requests || 0}</span>
      </div>
      <div class="stat-item">
        <span class="label">Success Rate</span>
        <span class="value">{proxyStats.success_rate || 0}%</span>
      </div>
      <div class="stat-item">
        <span class="label">Avg Response</span>
        <span class="value">{proxyStats.avg_response_time || 0}ms</span>
      </div>
    </div>
  {/if}

  <!-- Provider Fetching -->
  <div class="provider-section">
    <h4>Fetch From Provider</h4>
    <div class="provider-buttons">
      <button class="btn-provider" onclick={() => fetchFromProvider('free_proxy_list')} disabled={fetchingProxies}>
        FreeProxy
      </button>
      <button class="btn-provider" onclick={() => fetchFromProvider('proxy_scrape')} disabled={fetchingProxies}>
        ProxyScrape
      </button>
      <button class="btn-provider" onclick={() => fetchFromProvider('geonode')} disabled={fetchingProxies}>
        Geonode
      </button>
    </div>
  </div>

  <!-- Proxy Filters -->
  <div class="proxy-filters">
    <button class="filter-btn" class:active={proxyFilter === 'all'} onclick={() => proxyFilter = 'all'}>
      All
    </button>
    <button class="filter-btn" class:active={proxyFilter === 'http'} onclick={() => proxyFilter = 'http'}>
      HTTP
    </button>
    <button class="filter-btn" class:active={proxyFilter === 'socks5'} onclick={() => proxyFilter = 'socks5'}>
      SOCKS5
    </button>
  </div>

  <!-- Proxy List -->
  <div class="proxy-list">
    {#each filteredProxies() as proxy}
      <div 
        class="proxy-item" 
        class:active={currentProxy?.ip === proxy.ip}
        class:working={proxy.working}
        class:failed={proxy.failed}
        onclick={() => selectProxy(proxy)}
      >
        <div class="proxy-info">
          <span class="proxy-ip">{proxy.ip}:{proxy.port}</span>
          <span class="proxy-country">{proxy.country || 'Unknown'}</span>
          <span class="proxy-type">{proxy.protocol || 'HTTP'}</span>
          <span class="proxy-speed">{formatSpeed(proxy.speed)}</span>
        </div>
        <div class="proxy-actions-right">
          <button 
            class="validate-btn" 
            onclick={(e) => { e.stopPropagation(); validateProxy(proxy); }}
            disabled={validatingProxy}
          >
            Validate
          </button>
        </div>
      </div>
    {:else}
      <div class="no-proxies">
        No proxies available. Click "Fetch" to load proxies.
      </div>
    {/each}
  </div>
</div>

<style>
  .proxy-panel {
    background: #1a1a2e;
    border-radius: 8px;
    overflow: hidden;
  }

  .rotation-section {
    padding: 16px;
    border-bottom: 1px solid #2a2a45;
  }

  .rotation-section h4 {
    margin: 0 0 8px 0;
    font-size: 12px;
    font-weight: 600;
    color: #808090;
    text-transform: uppercase;
  }

  .strategy-select {
    width: 100%;
    padding: 8px 12px;
    background: #0d0d1a;
    border: 1px solid #2a2a45;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 13px;
    margin-bottom: 8px;
  }

  .btn-rotate {
    width: 100%;
    padding: 8px 16px;
    background: #4f46e5;
    border: none;
    border-radius: 6px;
    color: white;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-rotate:hover:not(:disabled) {
    background: #6366f1;
  }

  .btn-rotate:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .proxy-stats {
    padding: 12px 16px;
    background: #0d0d1a;
    border-bottom: 1px solid #2a2a45;
  }

  .stat-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 4px 0;
    font-size: 12px;
  }

  .stat-item .label {
    color: #808090;
  }

  .stat-item .value {
    color: #e0e0e0;
    font-family: monospace;
  }

  .provider-section {
    padding: 16px;
    border-bottom: 1px solid #2a2a45;
  }

  .provider-section h4 {
    margin: 0 0 8px 0;
    font-size: 12px;
    font-weight: 600;
    color: #808090;
    text-transform: uppercase;
  }

  .provider-buttons {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .btn-provider {
    padding: 6px 12px;
    background: #2a2a45;
    border: 1px solid #3a3a55;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 11px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-provider:hover:not(:disabled) {
    background: #3a3a55;
    border-color: #4a4a65;
  }

  .btn-provider:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .proxy-filters {
    display: flex;
    gap: 8px;
    padding: 12px 16px;
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

  .proxy-list {
    max-height: 400px;
    overflow-y: auto;
  }

  .proxy-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid #2a2a45;
    cursor: pointer;
    transition: all 0.2s;
  }

  .proxy-item:hover {
    background: #2a2a45;
  }

  .proxy-item.active {
    background: #2a2a45;
    border-left: 3px solid #4f46e5;
  }

  .proxy-item.working {
    border-left-color: #10b981;
  }

  .proxy-item.failed {
    border-left-color: #ef4444;
    opacity: 0.7;
  }

  .proxy-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .proxy-ip {
    font-family: monospace;
    color: #e0e0e0;
    font-size: 13px;
  }

  .proxy-country {
    font-size: 11px;
    color: #808090;
  }

  .proxy-type {
    font-size: 10px;
    color: #808090;
    text-transform: uppercase;
  }

  .proxy-speed {
    font-size: 11px;
    color: #10b981;
    font-family: monospace;
  }

  .proxy-actions-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .validate-btn {
    background: transparent;
    border: 1px solid #3a3a55;
    border-radius: 4px;
    color: #808090;
    cursor: pointer;
    padding: 4px 8px;
    font-size: 12px;
    transition: all 0.2s;
  }

  .validate-btn:hover:not(:disabled) {
    border-color: #4a4a65;
    color: #e0e0e0;
  }

  .validate-btn:disabled {
    cursor: not-allowed;
  }

  .no-proxies {
    text-align: center;
    padding: 20px;
    color: #808090;
    font-size: 13px;
  }
</style>
