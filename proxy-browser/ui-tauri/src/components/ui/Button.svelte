<script lang="ts">
  // Props
  let {
    variant = 'primary',
    size = 'medium',
    disabled = false,
    loading = false,
    fullWidth = false,
    type = 'button',
    onClick,
    children
  }: {
    variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
    size?: 'small' | 'medium' | 'large';
    disabled?: boolean;
    loading?: boolean;
    fullWidth?: boolean;
    type?: 'button' | 'submit' | 'reset';
    onClick?: (e: MouseEvent) => void;
    children?: any;
  } = $props();

  function handleClick(e: MouseEvent) {
    if (disabled || loading) return;
    if (onClick) onClick(e);
  }
</script>

<button
  class="btn btn-{variant} btn-{size}"
  class:full-width={fullWidth}
  class:loading
  {type}
  disabled={disabled || loading}
  onclick={handleClick}
>
  {#if loading}
    <span class="spinner"></span>
  {/if}
  <span class="btn-content" class:hidden={loading}>
    {@render children?.()}
  </span>
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    position: relative;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn.full-width {
    width: 100%;
  }

  /* Sizes */
  .btn-small {
    padding: 6px 12px;
    font-size: 12px;
  }

  .btn-medium {
    padding: 10px 16px;
    font-size: 14px;
  }

  .btn-large {
    padding: 12px 24px;
    font-size: 16px;
  }

  /* Variants */
  .btn-primary {
    background: #4f46e5;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #6366f1;
  }

  .btn-secondary {
    background: #2a2a45;
    color: #e0e0e0;
    border: 1px solid #3a3a55;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #3a3a55;
    border-color: #4a4a65;
  }

  .btn-danger {
    background: #ef4444;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #dc2626;
  }

  .btn-ghost {
    background: transparent;
    color: #e0e0e0;
  }

  .btn-ghost:hover:not(:disabled) {
    background: #2a2a45;
  }

  /* Loading state */
  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .btn-secondary .spinner,
  .btn-ghost .spinner {
    border-color: rgba(224, 224, 224, 0.3);
    border-top-color: #e0e0e0;
  }

  .btn-content.hidden {
    visibility: hidden;
  }

  .loading .spinner {
    position: absolute;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
