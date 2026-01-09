<script lang="ts">
  // Props
  let {
    open = $bindable(false),
    title = '',
    size = 'medium',
    closable = true,
    onClose,
    children
  }: {
    open?: boolean;
    title?: string;
    size?: 'small' | 'medium' | 'large' | 'fullscreen';
    closable?: boolean;
    onClose?: () => void;
    children?: any;
  } = $props();

  function close() {
    if (!closable) return;
    open = false;
    if (onClose) onClose();
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      close();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && closable) {
      close();
    }
  }

  $effect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
      document.addEventListener('keydown', handleKeydown);
    } else {
      document.body.style.overflow = '';
    }
    
    return () => {
      document.body.style.overflow = '';
      document.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

{#if open}
  <div class="modal-backdrop" onclick={handleBackdropClick}>
    <div class="modal modal-{size}" role="dialog" aria-modal="true">
      {#if title || closable}
        <div class="modal-header">
          {#if title}
            <h2 class="modal-title">{title}</h2>
          {/if}
          {#if closable}
            <button class="modal-close" onclick={close} aria-label="Close">
              ×
            </button>
          {/if}
        </div>
      {/if}
      
      <div class="modal-content">
        {@render children?.()}
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 20px;
    animation: fadeIn 0.2s ease;
  }

  .modal {
    background: #1a1a2e;
    border: 1px solid #2a2a45;
    border-radius: 12px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    animation: slideIn 0.2s ease;
  }

  .modal-small {
    width: 100%;
    max-width: 400px;
  }

  .modal-medium {
    width: 100%;
    max-width: 560px;
  }

  .modal-large {
    width: 100%;
    max-width: 800px;
  }

  .modal-fullscreen {
    width: 100%;
    height: 100%;
    max-width: none;
    max-height: none;
    border-radius: 0;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid #2a2a45;
  }

  .modal-title {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: #e0e0e0;
  }

  .modal-close {
    background: transparent;
    border: none;
    color: #808090;
    font-size: 24px;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    transition: color 0.2s;
  }

  .modal-close:hover {
    color: #e0e0e0;
  }

  .modal-content {
    padding: 20px;
    overflow-y: auto;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
