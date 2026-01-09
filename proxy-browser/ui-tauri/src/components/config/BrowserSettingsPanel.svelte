<script lang="ts">
  import { onMount } from 'svelte';
  import type { BrowserSettings } from '../../lib/types';
  import { getBrowserSettings, setBrowserSettings } from '../../lib/api';
  
  let settings: BrowserSettings = {
    user_agent: '',
    language: 'en-US',
    timezone: 'America/New_York',
    webrtc_policy: 'disable_non_proxied_udp',
    dns_over_https: true,
    block_trackers: true,
    block_ads: false,
    javascript_enabled: true,
    cookies_enabled: true,
    engine_type: 'system',
    stealth_mode: true,
    headless_mode: false,
  };
  
  let saving = false;
  let error: string | null = null;
  let success = false;
  
  const userAgents = [
    { name: 'Chrome (Windows)', value: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36' },
    { name: 'Chrome (Mac)', value: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36' },
    { name: 'Firefox (Windows)', value: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0' },
    { name: 'Safari (Mac)', value: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15' },
    { name: 'Edge (Windows)', value: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0' },
  ];
  
  const timezones = [
    'America/New_York', 'America/Chicago', 'America/Denver', 'America/Los_Angeles',
    'Europe/London', 'Europe/Paris', 'Europe/Berlin', 'Europe/Moscow',
    'Asia/Tokyo', 'Asia/Shanghai', 'Asia/Singapore', 'Asia/Dubai',
    'Australia/Sydney', 'Pacific/Auckland'
  ];
  
  onMount(async () => {
    try {
      settings = await getBrowserSettings();
    } catch (e) {
      error = 'Failed to load browser settings';
    }
  });
  
  async function handleSave() {
    saving = true;
    error = null;
    success = false;
    
    try {
      await setBrowserSettings(settings);
      success = true;
      setTimeout(() => success = false, 2000);
    } catch (e) {
      error = 'Failed to save settings';
    } finally {
      saving = false;
    }
  }
  
  function selectUserAgent(ua: string) {
    settings.user_agent = ua;
  }
</script>

<div class="config-section">
  <h3>Browser Settings</h3>
  
  <div class="form-group">
    <label>User Agent</label>
    <select on:change={(e) => selectUserAgent(e.currentTarget.value)}>
      <option value="">Select preset...</option>
      {#each userAgents as ua}
        <option value={ua.value}>{ua.name}</option>
      {/each}
    </select>
    <textarea 
      bind:value={settings.user_agent} 
      placeholder="Enter custom user agent..."
      rows="2"
    />
  </div>
  
  <div class="form-row">
    <div class="form-group">
      <label>Language</label>
      <input type="text" bind:value={settings.language} placeholder="en-US" />
    </div>
    <div class="form-group">
      <label>Timezone</label>
      <select bind:value={settings.timezone}>
        {#each timezones as tz}
          <option value={tz}>{tz}</option>
        {/each}
      </select>
    </div>
  </div>
  
  <div class="form-group">
    <label>WebRTC Policy</label>
    <select bind:value={settings.webrtc_policy}>
      <option value="default">Default (may leak IP)</option>
      <option value="disable_non_proxied_udp">Disable Non-Proxied UDP (recommended)</option>
      <option value="disabled">Completely Disabled</option>
    </select>
    <span class="hint">Controls WebRTC behavior to prevent IP leaks</span>
  </div>
  
  <div class="toggles">
    <label class="toggle-label">
      <input type="checkbox" bind:checked={settings.dns_over_https} />
      <span>DNS over HTTPS</span>
    </label>
    <label class="toggle-label">
      <input type="checkbox" bind:checked={settings.block_trackers} />
      <span>Block Trackers</span>
    </label>
    <label class="toggle-label">
      <input type="checkbox" bind:checked={settings.block_ads} />
      <span>Block Ads</span>
    </label>
    <label class="toggle-label">
      <input type="checkbox" bind:checked={settings.javascript_enabled} />
      <span>JavaScript Enabled</span>
    </label>
    <label class="toggle-label">
      <input type="checkbox" bind:checked={settings.cookies_enabled} />
      <span>Cookies Enabled</span>
    </label>
  </div>
  
  <div class="engine-section">
    <h4>Browser Engine</h4>
    <div class="engine-toggle">
      <label class="engine-option" class:active={settings.engine_type === 'system'}>
        <input 
          type="radio" 
          name="engine_type" 
          value="system" 
          bind:group={settings.engine_type} 
        />
        <div class="engine-info">
          <span class="engine-name">System WebView</span>
          <span class="engine-desc">Uses Tauri's built-in webview. Lighter weight, uses system resources.</span>
        </div>
      </label>
      <label class="engine-option" class:active={settings.engine_type === 'integrated_chromium'}>
        <input 
          type="radio" 
          name="engine_type" 
          value="integrated_chromium" 
          bind:group={settings.engine_type} 
        />
        <div class="engine-info">
          <span class="engine-name">Integrated Chromium</span>
          <span class="engine-desc">Full Chromium engine with enhanced proxy support, stealth mode, and WebRTC protection.</span>
          <span class="engine-badge">Recommended for Proxies</span>
        </div>
      </label>
    </div>
    
    {#if settings.engine_type === 'integrated_chromium'}
      <div class="chromium-options">
        <label class="toggle-label">
          <input type="checkbox" bind:checked={settings.stealth_mode} />
          <span>Stealth Mode</span>
          <span class="hint">Avoid bot detection by hiding automation signals</span>
        </label>
        <label class="toggle-label">
          <input type="checkbox" bind:checked={settings.headless_mode} />
          <span>Headless Mode</span>
          <span class="hint">Run browser without visible window (background mode)</span>
        </label>
      </div>
    {/if}
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
  
  label {
    display: block;
    font-size: 12px;
    color: #9fb0ce;
    margin-bottom: 4px;
  }
  
  input, select, textarea {
    width: 100%;
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    padding: 8px 10px;
    color: #e0e7f5;
    font-size: 13px;
    font-family: inherit;
  }
  
  textarea {
    resize: vertical;
    margin-top: 6px;
  }
  
  input:focus, select:focus, textarea:focus {
    outline: none;
    border-color: #3b82f6;
  }
  
  .hint {
    display: block;
    font-size: 11px;
    color: #6b7a9a;
    margin-top: 4px;
  }
  
  .toggles {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin: 16px 0;
  }
  
  .toggle-label {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
    color: #c7d4ec;
    cursor: pointer;
  }
  
  .toggle-label input {
    width: auto;
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
  
  .engine-section {
    margin-top: 20px;
    padding-top: 16px;
    border-top: 1px solid #1f2a45;
  }
  
  .engine-section h4 {
    margin: 0 0 12px;
    font-size: 13px;
    color: #c7d4ec;
  }
  
  .engine-toggle {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  
  .engine-option {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px;
    background: #0c1120;
    border: 2px solid #1f2a45;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  
  .engine-option:hover {
    border-color: #3b82f6;
  }
  
  .engine-option.active {
    border-color: #3b82f6;
    background: #0f1a30;
  }
  
  .engine-option input[type="radio"] {
    width: auto;
    margin-top: 3px;
  }
  
  .engine-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  
  .engine-name {
    font-size: 14px;
    font-weight: 600;
    color: #e0e7f5;
  }
  
  .engine-desc {
    font-size: 12px;
    color: #9fb0ce;
    line-height: 1.4;
  }
  
  .engine-badge {
    display: inline-block;
    margin-top: 4px;
    padding: 2px 8px;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    color: white;
    font-size: 10px;
    font-weight: 600;
    border-radius: 4px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    width: fit-content;
  }
  
  .chromium-options {
    margin-top: 12px;
    padding: 12px;
    background: #0a0e18;
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  
  .chromium-options .toggle-label {
    flex-wrap: wrap;
  }
  
  .chromium-options .hint {
    flex-basis: 100%;
    margin-left: 28px;
  }
</style>
