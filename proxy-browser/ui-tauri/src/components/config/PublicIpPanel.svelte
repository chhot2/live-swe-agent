<script lang="ts">
  import { onMount } from 'svelte';
  import type { PublicIpInfo } from '../../lib/types';
  import { detectPublicIp } from '../../lib/api';
  
  let ipInfo: PublicIpInfo | null = null;
  let loading = false;
  let error: string | null = null;
  let autoRefresh = false;
  let refreshInterval: number | null = null;
  
  onMount(() => {
    refresh();
    return () => {
      if (refreshInterval) clearInterval(refreshInterval);
    };
  });
  
  async function refresh() {
    loading = true;
    error = null;
    try {
      ipInfo = await detectPublicIp();
    } catch (e) {
      error = 'Failed to detect IP';
    } finally {
      loading = false;
    }
  }
  
  function toggleAutoRefresh() {
    autoRefresh = !autoRefresh;
    if (autoRefresh) {
      refreshInterval = setInterval(refresh, 30000) as unknown as number;
    } else if (refreshInterval) {
      clearInterval(refreshInterval);
      refreshInterval = null;
    }
  }
</script>

<div class="config-section">
  <div class="section-header">
    <h3>Public IP</h3>
    <button class="btn-icon" on:click={refresh} disabled={loading} title="Refresh">
      <span class:spinning={loading}>↻</span>
    </button>
  </div>
  
  {#if loading && !ipInfo}
    <div class="loading">Detecting IP...</div>
  {:else if ipInfo}
    <div class="ip-display">
      <div class="ip-main">{ipInfo.ip}</div>
      <div class="ip-details">
        {#if ipInfo.country}
          <span class="detail">
            <strong>Country:</strong> {ipInfo.country}
            {#if ipInfo.country_code}({ipInfo.country_code}){/if}
          </span>
        {/if}
        {#if ipInfo.city}
          <span class="detail"><strong>City:</strong> {ipInfo.city}</span>
        {/if}
        {#if ipInfo.isp}
          <span class="detail"><strong>ISP:</strong> {ipInfo.isp}</span>
        {/if}
        {#if ipInfo.timezone}
          <span class="detail"><strong>Timezone:</strong> {ipInfo.timezone}</span>
        {/if}
      </div>
    </div>
    
    <label class="checkbox-label">
      <input type="checkbox" checked={autoRefresh} on:change={toggleAutoRefresh} />
      Auto-refresh every 30s
    </label>
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
  
  .btn-icon {
    background: transparent;
    border: 1px solid #1f2a45;
    color: #9fb0ce;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
  }
  
  .btn-icon:hover {
    background: #1f2a45;
  }
  
  .spinning {
    display: inline-block;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  .loading {
    color: #9fb0ce;
    font-size: 13px;
  }
  
  .ip-display {
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 8px;
    padding: 12px;
  }
  
  .ip-main {
    font-size: 20px;
    font-weight: 600;
    color: #3b82f6;
    margin-bottom: 8px;
    font-family: monospace;
  }
  
  .ip-details {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  
  .detail {
    font-size: 12px;
    color: #9fb0ce;
  }
  
  .detail strong {
    color: #c7d4ec;
  }
  
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 12px;
    font-size: 12px;
    color: #9fb0ce;
    cursor: pointer;
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
