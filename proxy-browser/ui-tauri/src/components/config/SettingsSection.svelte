<script lang="ts">
  // Props
  let {
    title,
    description = '',
    collapsed = $bindable(false),
    collapsible = true,
    children
  }: {
    title: string;
    description?: string;
    collapsed?: boolean;
    collapsible?: boolean;
    children?: any;
  } = $props();

  function toggleCollapsed() {
    if (collapsible) {
      collapsed = !collapsed;
    }
  }
</script>

<div class="settings-section" class:collapsed>
  <button 
    class="section-header" 
    onclick={toggleCollapsed}
    disabled={!collapsible}
  >
    <div class="header-content">
      <h3 class="section-title">{title}</h3>
      {#if description}
        <p class="section-description">{description}</p>
      {/if}
    </div>
    {#if collapsible}
      <span class="collapse-icon">{collapsed ? '▶' : '▼'}</span>
    {/if}
  </button>
  
  {#if !collapsed}
    <div class="section-content">
      {@render children?.()}
    </div>
  {/if}
</div>

<style>
  .settings-section {
    background: #1a1a2e;
    border: 1px solid #2a2a45;
    border-radius: 8px;
    overflow: hidden;
    margin-bottom: 16px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 16px;
    background: transparent;
    border: none;
    cursor: pointer;
    text-align: left;
    transition: background 0.2s;
  }

  .section-header:hover:not(:disabled) {
    background: #2a2a45;
  }

  .section-header:disabled {
    cursor: default;
  }

  .header-content {
    flex: 1;
  }

  .section-title {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: #e0e0e0;
  }

  .section-description {
    margin: 4px 0 0 0;
    font-size: 12px;
    color: #808090;
  }

  .collapse-icon {
    color: #808090;
    font-size: 12px;
    transition: transform 0.2s;
  }

  .section-content {
    padding: 0 16px 16px 16px;
    border-top: 1px solid #2a2a45;
  }

  .collapsed .section-content {
    display: none;
  }
</style>
