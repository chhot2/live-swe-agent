<script lang="ts">
  interface Option {
    value: string;
    label: string;
    disabled?: boolean;
  }

  // Props
  let {
    options = [],
    value = $bindable(''),
    placeholder = 'Select an option',
    disabled = false,
    label = '',
    error = '',
    onChange
  }: {
    options: Option[];
    value?: string;
    placeholder?: string;
    disabled?: boolean;
    label?: string;
    error?: string;
    onChange?: (value: string) => void;
  } = $props();

  function handleChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    value = target.value;
    if (onChange) onChange(value);
  }
</script>

<div class="select-wrapper" class:has-error={!!error}>
  {#if label}
    <label class="select-label">{label}</label>
  {/if}
  
  <div class="select-container">
    <select
      class="select-input"
      bind:value
      onchange={handleChange}
      {disabled}
    >
      {#if placeholder}
        <option value="" disabled>{placeholder}</option>
      {/if}
      {#each options as option}
        <option value={option.value} disabled={option.disabled}>
          {option.label}
        </option>
      {/each}
    </select>
    <span class="select-arrow">▼</span>
  </div>
  
  {#if error}
    <span class="error-message">{error}</span>
  {/if}
</div>

<style>
  .select-wrapper {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .select-label {
    font-size: 13px;
    font-weight: 500;
    color: #e0e0e0;
  }

  .select-container {
    position: relative;
    display: flex;
    align-items: center;
  }

  .select-input {
    width: 100%;
    padding: 10px 36px 10px 12px;
    background: #0d0d1a;
    border: 1px solid #2a2a45;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 14px;
    cursor: pointer;
    appearance: none;
    transition: all 0.2s;
  }

  .select-input:focus {
    outline: none;
    border-color: #4f46e5;
    box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.1);
  }

  .select-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .select-wrapper.has-error .select-input {
    border-color: #ef4444;
  }

  .select-arrow {
    position: absolute;
    right: 12px;
    color: #808090;
    font-size: 10px;
    pointer-events: none;
  }

  .error-message {
    font-size: 12px;
    color: #ef4444;
  }

  option {
    background: #1a1a2e;
    color: #e0e0e0;
  }
</style>
