<script lang="ts">
  // Props
  let {
    checked = $bindable(false),
    disabled = false,
    label = '',
    description = '',
    onChange
  }: {
    checked?: boolean;
    disabled?: boolean;
    label?: string;
    description?: string;
    onChange?: (checked: boolean) => void;
  } = $props();

  function toggle() {
    if (disabled) return;
    checked = !checked;
    if (onChange) onChange(checked);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      toggle();
    }
  }
</script>

<div class="toggle-wrapper" class:disabled>
  <button
    class="toggle-switch"
    class:checked
    onclick={toggle}
    onkeydown={handleKeydown}
    {disabled}
    role="switch"
    aria-checked={checked}
  >
    <span class="toggle-slider"></span>
  </button>
  
  {#if label || description}
    <div class="toggle-label" onclick={toggle}>
      {#if label}
        <span class="label-text">{label}</span>
      {/if}
      {#if description}
        <span class="label-description">{description}</span>
      {/if}
    </div>
  {/if}
</div>

<style>
  .toggle-wrapper {
    display: flex;
    align-items: flex-start;
    gap: 12px;
  }

  .toggle-wrapper.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toggle-switch {
    position: relative;
    width: 44px;
    height: 24px;
    background: #2a2a45;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
    flex-shrink: 0;
    padding: 0;
  }

  .toggle-switch:hover:not(:disabled) {
    background: #3a3a55;
  }

  .toggle-switch.checked {
    background: #4f46e5;
  }

  .toggle-switch:disabled {
    cursor: not-allowed;
  }

  .toggle-slider {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    background: white;
    border-radius: 50%;
    transition: transform 0.2s;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .toggle-switch.checked .toggle-slider {
    transform: translateX(20px);
  }

  .toggle-label {
    display: flex;
    flex-direction: column;
    gap: 2px;
    cursor: pointer;
  }

  .toggle-wrapper.disabled .toggle-label {
    cursor: not-allowed;
  }

  .label-text {
    font-size: 14px;
    font-weight: 500;
    color: #e0e0e0;
  }

  .label-description {
    font-size: 12px;
    color: #808090;
  }
</style>
