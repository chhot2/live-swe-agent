<script lang="ts">
  import { onMount } from 'svelte';
  import type { ProxySettings } from '../../lib/types';
  import { getProxySettings, setProxySettings } from '../../lib/api';
  
  let settings: ProxySettings = {
    proxy_type: 'direct',
    host: null,
    port: null,
    username: null,
    password: null,
    dns_servers: ['1.1.1.1', '8.8.8.8'],
    bypass_list: ['localhost', '127.0.0.1'],
  };
  
  let saving = false;
  let error: string | null = null;
  let success = false;
  
  let dnsInput = '';
  let bypassInput = '';
  
  onMount(async () => {
    try {
      settings = await getProxySettings();
      dnsInput = settings.dns_servers.join(', ');
      bypassInput = settings.bypass_list.join(', ');
    } catch (e) {
      error = 'Failed to load proxy settings';
    }
  });
  
  async function handleSave() {
    saving = true;
    error = null;
    success = false;
    
    try {
      settings.dns_servers = dnsInput.split(',').map(s => s.trim()).filter(Boolean);
      settings.bypass_list = bypassInput.split(',').map(s => s.trim()).filter(Boolean);
      await setProxySettings(settings);
      success = true;
      setTimeout(() => success = false, 2000);
    } catch (e) {
      error = 'Failed to save settings';
    } finally {
      saving = false;
    }
  }
</script>

<div class="config-section">
  <h3>Proxy Configuration</h3>
  
  <div class="form-group">
    <label>Proxy Type</label>
    <select bind:value={settings.proxy_type}>
      <option value="direct">Direct Connection</option>
      <option value="http">HTTP Proxy</option>
      <option value="https">HTTPS Proxy</option>
      <option value="socks4">SOCKS4 Proxy</option>
      <option value="socks5">SOCKS5 Proxy</option>
    </select>
  </div>
  
  {#if settings.proxy_type !== 'direct'}
    <div class="form-row">
      <div class="form-group flex-2">
        <label>Host</label>
        <input type="text" bind:value={settings.host} placeholder="proxy.example.com" />
      </div>
      <div class="form-group flex-1">
        <label>Port</label>
        <input type="number" bind:value={settings.port} placeholder="8080" />
      </div>
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label>Username (optional)</label>
        <input type="text" bind:value={settings.username} placeholder="username" />
      </div>
      <div class="form-group">
        <label>Password (optional)</label>
        <input type="password" bind:value={settings.password} placeholder="password" />
      </div>
    </div>
  {/if}
  
  <div class="form-group">
    <label>DNS Servers (comma separated)</label>
    <input type="text" bind:value={dnsInput} placeholder="1.1.1.1, 8.8.8.8" />
  </div>
  
  <div class="form-group">
    <label>Bypass List (comma separated)</label>
    <input type="text" bind:value={bypassInput} placeholder="localhost, 127.0.0.1" />
  </div>
  
  <div class="actions">
    <button class="btn-primary" on:click={handleSave} disabled={saving}>
      {saving ? 'Saving...' : 'Save Settings'}
    </button>
  </div>
  
  {#if error}
    <div class="error-msg">{error}</div>
  {/if}
  
  {#if success}
    <div class="success-msg">Settings saved!</div>
  {/if}
</div>

<style>
  .config-section {
    padding: 16px;
    border-bottom: 1px solid #1f2a45;
  }
  
  h3 {
    margin: 0 0 16px;
    font-size: 14px;
    color: #c7d4ec;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .form-group {
    margin-bottom: 12px;
  }
  
  .form-row {
    display: flex;
    gap: 12px;
  }
  
  .form-row .form-group {
    flex: 1;
  }
  
  .flex-2 { flex: 2 !important; }
  .flex-1 { flex: 1 !important; }
  
  label {
    display: block;
    font-size: 12px;
    color: #9fb0ce;
    margin-bottom: 4px;
  }
  
  input, select {
    width: 100%;
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    padding: 8px 10px;
    color: #e0e7f5;
    font-size: 13px;
  }
  
  input:focus, select:focus {
    outline: none;
    border-color: #3b82f6;
  }
  
  .actions {
    margin-top: 16px;
  }
  
  .btn-primary {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
  }
  
  .btn-primary:hover {
    background: #2563eb;
  }
  
  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
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
  
  .success-msg {
    margin-top: 12px;
    padding: 8px 12px;
    background: #1a3a2c;
    border: 1px solid #5cff8a;
    color: #b3ffc8;
    border-radius: 6px;
    font-size: 12px;
  }
</style>
