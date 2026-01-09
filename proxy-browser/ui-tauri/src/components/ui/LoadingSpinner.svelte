<script lang="ts">
  export let size: 'small' | 'medium' | 'large' = 'medium';
  export let color: string = '#3b82f6';
</script>

<div class="spinner" class:size style="--spinner-color: {color};">
  <div class="circle"></div>
</div>

<style>
  .spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }
  
  .spinner.small {
    width: 16px;
    height: 16px;
  }
  
  .spinner.medium {
    width: 24px;
    height: 24px;
  }
  
  .spinner.large {
    width: 32px;
    height: 32px;
  }
  
  .circle {
    width: 100%;
    height: 100%;
    border: 2px solid transparent;
    border-top-color: var(--spinner-color);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
