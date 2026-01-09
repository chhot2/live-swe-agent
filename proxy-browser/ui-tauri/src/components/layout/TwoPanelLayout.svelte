<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let leftWidth = 60;
  
  let isDragging = false;
  let container: HTMLElement;
  
  const dispatch = createEventDispatcher();
  
  function startDrag(e: MouseEvent) {
    isDragging = true;
    e.preventDefault();
  }
  
  function onDrag(e: MouseEvent) {
    if (!isDragging || !container) return;
    const rect = container.getBoundingClientRect();
    const newWidth = ((e.clientX - rect.left) / rect.width) * 100;
    leftWidth = Math.max(30, Math.min(70, newWidth));
  }
  
  function stopDrag() {
    isDragging = false;
  }
</script>

<svelte:window on:mousemove={onDrag} on:mouseup={stopDrag} />

<div class="layout" bind:this={container}>
  <div class="left-panel" style="width: {leftWidth}%">
    <slot name="left" />
  </div>
  
  <div 
    class="divider" 
    class:dragging={isDragging}
    on:mousedown={startDrag}
    role="separator"
    tabindex="0"
  />
  
  <div class="right-panel" style="width: {100 - leftWidth}%">
    <slot name="right" />
  </div>
</div>

<style>
  .layout {
    display: flex;
    height: 100%;
    width: 100%;
    overflow: hidden;
  }
  
  .left-panel, .right-panel {
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
  }
  
  .left-panel {
    background: rgba(15, 21, 39, 0.9);
    border-right: 1px solid #1f2a45;
  }
  
  .right-panel {
    background: rgba(12, 17, 32, 0.95);
  }
  
  .divider {
    width: 6px;
    background: #1f2a45;
    cursor: col-resize;
    transition: background 0.2s;
    flex-shrink: 0;
  }
  
  .divider:hover, .divider.dragging {
    background: #3b82f6;
  }
</style>
