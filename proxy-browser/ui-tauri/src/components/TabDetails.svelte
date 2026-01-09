<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Tab, Country, ValidationResponse } from '../lib/types';

  export let tab: Tab;
  export let countries: Country[] = [];
  export let validation: ValidationResponse | null = null;

  const dispatch = createEventDispatcher<{ rotate: { country?: string }, validate: void }>();
  let rotateCountry = '';

  function rotate() {
    dispatch('rotate', { country: rotateCountry || undefined });
  }
</script>

<div class="details">
  <div class="header">
    <div class="flag">{tab.country_code === 'US' ? '🇺🇸' : '🌍'}</div>
    <div>
      <h2>{tab.country_name}</h2>
      <p>{tab.city || 'Unknown'}, {tab.timezone}</p>
    </div>
  </div>

  <div class="grid">
    <div class="card">
      <div class="label">IP Address</div>
      <code>{tab.ip}</code>
    </div>
    <div class="card">
      <div class="label">ISP</div>
      <div>{tab.isp || 'Unknown'}</div>
    </div>
    <div class="card">
      <div class="label">Country</div>
      <div>{tab.country_code} – {tab.country_name}</div>
    </div>
    <div class="card">
      <div class="label">Timezone</div>
      <div>{tab.timezone}</div>
    </div>
  </div>

  <div class="actions">
    <div class="row">
      <select bind:value={rotateCountry}>
        <option value=''>Same country</option>
        {#each countries as c}
          <option value={c.code}>{c.flag || '🌐'} {c.name}</option>
        {/each}
      </select>
      <button class="primary" on:click={rotate}>🔄 Rotate IP</button>
      <button class="ghost" on:click={() => dispatch('validate')}>✓ Validate</button>
    </div>
  </div>

  {#if validation}
    <div class="validation">
      <h3>Validation</h3>
      <div class="row">
        <span>IP Match</span>
        <span class={validation.ip_matches ? 'ok' : 'fail'}>{validation.ip_matches ? 'Pass' : 'Fail'}</span>
      </div>
      <div class="row">
        <span>WebRTC Secure</span>
        <span class={validation.webrtc_secure ? 'ok' : 'fail'}>{validation.webrtc_secure ? 'Pass' : 'Fail'}</span>
      </div>
      <div class="row">
        <span>DNS Secure</span>
        <span class={validation.dns_secure ? 'ok' : 'fail'}>{validation.dns_secure ? 'Pass' : 'Fail'}</span>
      </div>
      <div class="row total">
        <span>Overall</span>
        <span class={validation.overall_pass ? 'ok' : 'fail'}>{validation.overall_pass ? 'Pass' : 'Fail'}</span>
      </div>
    </div>
  {/if}
</div>

<style>
  .details {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .header {
    display: flex;
    gap: 12px;
    align-items: center;
  }
  .flag {
    font-size: 32px;
  }
  h2 {
    margin: 0;
    font-size: 22px;
  }
  p {
    margin: 2px 0 0;
    color: #9fb0ce;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 10px;
  }
  .card {
    background: #0f1729;
    border: 1px solid #1f2e4a;
    border-radius: 12px;
    padding: 10px;
  }
  .label {
    font-size: 12px;
    color: #8fa1c4;
    margin-bottom: 4px;
  }
  code {
    font-size: 14px;
    color: #cde3ff;
  }
  .actions {
    background: #0f1729;
    border: 1px solid #1f2e4a;
    border-radius: 12px;
    padding: 10px;
  }
  .row {
    display: flex;
    gap: 8px;
    align-items: center;
    flex-wrap: wrap;
  }
  select {
    background: #0b1324;
    color: #e8ecf3;
    border: 1px solid #243556;
    border-radius: 8px;
    padding: 8px 10px;
  }
  button {
    border: none;
    border-radius: 10px;
    padding: 10px 12px;
    cursor: pointer;
    font-weight: 600;
    color: #e8ecf3;
    transition: transform 0.1s ease, opacity 0.15s ease;
  }
  button:hover {
    transform: translateY(-1px);
  }
  .primary {
    background: linear-gradient(120deg, #2563eb, #1d4ed8);
    border: 1px solid #2b57c2;
  }
  .ghost {
    background: #10182c;
    border: 1px solid #2d3d5c;
  }
  .validation {
    background: #0f1729;
    border: 1px solid #1f2e4a;
    border-radius: 12px;
    padding: 10px;
  }
  .validation h3 {
    margin: 0 0 8px;
  }
  .validation .row {
    justify-content: space-between;
  }
  .ok {
    color: #8ae29f;
  }
  .fail {
    color: #ff9fb0;
  }
  .total {
    font-weight: 700;
  }
</style>
