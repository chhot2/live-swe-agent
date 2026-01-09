<script lang="ts">
  import ProxyConfiguration from '../config/ProxyConfiguration.svelte';
  import PublicIpPanel from '../config/PublicIpPanel.svelte';
  import FreeProxiesPanel from '../config/FreeProxiesPanel.svelte';
  import BrowserSettingsPanel from '../config/BrowserSettingsPanel.svelte';
  import BackupRestorePanel from '../config/BackupRestorePanel.svelte';
  
  let activeSection = 'proxy';
</script>

<div class="right-panel">
  <div class="panel-header">
    <h2>Configuration</h2>
  </div>
  
  <div class="tabs">
    <button 
      class="tab" 
      class:active={activeSection === 'proxy'}
      on:click={() => activeSection = 'proxy'}
    >
      Proxy
    </button>
    <button 
      class="tab" 
      class:active={activeSection === 'ip'}
      on:click={() => activeSection = 'ip'}
    >
      IP Info
    </button>
    <button 
      class="tab" 
      class:active={activeSection === 'free'}
      on:click={() => activeSection = 'free'}
    >
      Free Proxies
    </button>
    <button 
      class="tab" 
      class:active={activeSection === 'browser'}
      on:click={() => activeSection = 'browser'}
    >
      Browser
    </button>
    <button 
      class="tab" 
      class:active={activeSection === 'backup'}
      on:click={() => activeSection = 'backup'}
    >
      Backup
    </button>
  </div>
  
  <div class="panel-content">
    {#if activeSection === 'proxy'}
      <ProxyConfiguration />
    {:else if activeSection === 'ip'}
      <PublicIpPanel />
    {:else if activeSection === 'free'}
      <FreeProxiesPanel />
    {:else if activeSection === 'browser'}
      <BrowserSettingsPanel />
    {:else if activeSection === 'backup'}
      <BackupRestorePanel />
    {/if}
  </div>
</div>

<style>
  .right-panel {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .panel-header {
    padding: 16px;
    border-bottom: 1px solid #1f2a45;
  }
  
  .panel-header h2 {
    margin: 0;
    font-size: 18px;
    color: #e0e7f5;
  }
  
  .tabs {
    display: flex;
    border-bottom: 1px solid #1f2a45;
    padding: 0 8px;
  }
  
  .tab {
    background: transparent;
    border: none;
    color: #9fb0ce;
    padding: 12px 16px;
    font-size: 13px;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
  }
  
  .tab:hover {
    color: #e0e7f5;
  }
  
  .tab.active {
    color: #3b82f6;
    border-bottom-color: #3b82f6;
  }
  
  .panel-content {
    flex: 1;
    overflow-y: auto;
  }
</style>
