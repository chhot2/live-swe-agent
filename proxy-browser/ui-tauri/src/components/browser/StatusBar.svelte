<script lang="ts">
  import type { PublicIpInfo, FreeProxy } from '../../lib/types';

  // Props
  let {
    publicIp,
    currentProxy,
    isLoading = false,
    connectionStatus = 'connected'
  }: {
    publicIp: PublicIpInfo | null;
    currentProxy: FreeProxy | null;
    isLoading?: boolean;
    connectionStatus?: 'connected' | 'disconnected' | 'connecting';
  } = $props();

  // Derived state
  let statusColor = $derived(() => {
    switch (connectionStatus) {
      case 'connected': return '#10b981';
      case 'disconnected': return '#ef4444';
      case 'connecting': return '#f59e0b';
      default: return '#808090';
    }
  });

  let proxyInfo = $derived(() => {
    if (!currentProxy) return 'Direct Connection';
    return `${currentProxy.ip}:${currentProxy.port} (${currentProxy.country || 'Unknown'})`;
  });

  let ipInfo = $derived(() => {
    if (!publicIp) return 'Detecting...';
    return `${publicIp.ip} - ${publicIp.country || 'Unknown'}`;
  });
</script>

<div class="status-bar">
  <div class="status-left">
    <div class="status-item">
      <span class="status-dot" style="background-color: {statusColor()}"></span>
      <span class="status-text">{connectionStatus}</span>
    </div>
    
    {#if isLoading}
      <div class="status-item loading">
        <span class="loading-spinner"></span>
        <span class="status-text">Loading...</span>
      </div>
    {/if}
  </div>
  
  <div class="status-center">
    <div class="status-item" title="Current Proxy">
      <span class="status-icon">🔒</span>
      <span class="status-text">{proxyInfo()}</span>
    </div>
  </div>
  
  <div class="status-right">
    <div class="status-item" title="Public IP">
      <span class="status-icon">🌐</span>
      <span class="status-text">{ipInfo()}</span>
    </div>
  </div>
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 12px;
    background: #0d0d1a;
    border-top: 1px solid #2a2a45;
    font-size: 11px;
    color: #808090;
    min-height: 24px;
  }

  .status-left, .status-center, .status-right {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .status-item {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-icon {
    font-size: 12px;
  }

  .status-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 200px;
  }

  .loading-spinner {
    width: 12px;
    height: 12px;
    border: 2px solid #3a3a55;
    border-top-color: #4f46e5;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
