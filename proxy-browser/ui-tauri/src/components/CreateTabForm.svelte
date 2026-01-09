<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Country } from '../lib/types';

  export let countries: Country[] = [];
  const dispatch = createEventDispatcher<{ createTab: { countryCode: string }, createRandom: void }>();

  let selected = '';

  function submit() {
    const code = selected || (countries.find((c) => c.is_top)?.code ?? '');
    if (code) dispatch('createTab', { countryCode: code });
  }
</script>

<div class="form">
  <div class="row">
    <label>Country</label>
    <select bind:value={selected}>
      <option value="" disabled selected>Select country</option>
      {#each countries as c}
        <option value={c.code}>{c.flag || '🌐'} {c.name}</option>
      {/each}
    </select>
  </div>
  <div class="actions">
    <button class="primary" on:click={submit}>➕ Create Tab</button>
    <button class="ghost" on:click={() => dispatch('createRandom')}>🎲 Random Country</button>
  </div>
</div>

<style>
  .form {
    background: #0f1729;
    border: 1px solid #1f2e4a;
    border-radius: 12px;
    padding: 12px;
    margin-bottom: 12px;
  }
  .row {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  label {
    font-size: 13px;
    color: #b8c8e6;
  }
  select {
    background: #0b1324;
    color: #e8ecf3;
    border: 1px solid #243556;
    border-radius: 8px;
    padding: 8px 10px;
  }
  .actions {
    display: flex;
    gap: 8px;
    margin-top: 10px;
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
</style>
