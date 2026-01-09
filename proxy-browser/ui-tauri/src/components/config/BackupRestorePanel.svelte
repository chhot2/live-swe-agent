<script lang="ts">
  import { onMount } from 'svelte';
  import type { BackupInfo, BackupOptions } from '../../lib/types';
  import { createBackup, listBackups, restoreBackup, deleteBackup } from '../../lib/api';
  
  let backups: BackupInfo[] = [];
  let loading = false;
  let creating = false;
  let error: string | null = null;
  let success: string | null = null;
  
  let options: BackupOptions = {
    include_proxy_settings: true,
    include_browser_config: true,
    include_cookies: true,
    include_history: true,
    include_bookmarks: true,
    include_local_storage: true,
    password: null,
  };
  
  let usePassword = false;
  let restorePassword = '';
  
  onMount(async () => {
    await loadBackups();
  });
  
  async function loadBackups() {
    try {
      backups = await listBackups();
    } catch (e) {
      // No backups
    }
  }
  
  async function handleCreate() {
    creating = true;
    error = null;
    success = null;
    
    try {
      const backupOptions = { ...options };
      if (!usePassword) backupOptions.password = null;
      
      const info = await createBackup(backupOptions);
      success = `Backup created: ${info.filename}`;
      await loadBackups();
    } catch (e) {
      error = 'Failed to create backup';
    } finally {
      creating = false;
    }
  }
  
  async function handleRestore(backup: BackupInfo) {
    loading = true;
    error = null;
    success = null;
    
    try {
      const password = backup.is_encrypted ? restorePassword : undefined;
      await restoreBackup(backup.path, password);
      success = 'Backup restored successfully';
    } catch (e) {
      error = 'Failed to restore backup. Wrong password?';
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete(backup: BackupInfo) {
    try {
      await deleteBackup(backup.id);
      await loadBackups();
    } catch (e) {
      error = 'Failed to delete backup';
    }
  }
  
  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
  
  function formatDate(iso: string): string {
    const date = new Date(iso);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
  }
</script>

<div class="config-section">
  <h3>Backup & Restore</h3>
  
  <div class="backup-options">
    <h4>Create Backup</h4>
    
    <div class="checkbox-grid">
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={options.include_proxy_settings} />
        Proxy Settings
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={options.include_cookies} />
        Cookies
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={options.include_history} />
        History
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={options.include_bookmarks} />
        Bookmarks
      </label>
    </div>
    
    <label class="checkbox-label">
      <input type="checkbox" bind:checked={usePassword} />
      Password protect backup
    </label>
    
    {#if usePassword}
      <input 
        type="password" 
        bind:value={options.password} 
        placeholder="Enter password..."
        class="password-input"
      />
    {/if}
    
    <button class="btn-primary" on:click={handleCreate} disabled={creating}>
      {creating ? 'Creating...' : 'Create Backup'}
    </button>
  </div>
  
  <div class="backup-list">
    <h4>Existing Backups</h4>
    
    {#each backups as backup}
      <div class="backup-item">
        <div class="backup-info">
          <span class="backup-name">{backup.filename}</span>
          <span class="backup-meta">
            {formatSize(backup.size_bytes)} • {formatDate(backup.created_at)}
            {#if backup.is_encrypted}
              <span class="encrypted">🔒</span>
            {/if}
          </span>
        </div>
        <div class="backup-actions">
          {#if backup.is_encrypted}
            <input 
              type="password" 
              bind:value={restorePassword}
              placeholder="Password"
              class="restore-password"
            />
          {/if}
          <button class="btn-sm" on:click={() => handleRestore(backup)} disabled={loading}>
            Restore
          </button>
          <button class="btn-sm danger" on:click={() => handleDelete(backup)}>
            Delete
          </button>
        </div>
      </div>
    {/each}
    
    {#if backups.length === 0}
      <div class="empty">No backups found</div>
    {/if}
  </div>
  
  {#if error}
    <div class="error-msg">{error}</div>
  {/if}
  
  {#if success}
    <div class="success-msg">{success}</div>
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
  
  h4 {
    margin: 0 0 12px;
    font-size: 12px;
    color: #9fb0ce;
  }
  
  .backup-options {
    margin-bottom: 20px;
  }
  
  .checkbox-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    margin-bottom: 12px;
  }
  
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: #9fb0ce;
    cursor: pointer;
  }
  
  .password-input, .restore-password {
    width: 100%;
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    padding: 8px 10px;
    color: #e0e7f5;
    font-size: 12px;
    margin: 8px 0;
  }
  
  .restore-password {
    width: 100px;
  }
  
  .btn-primary {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    margin-top: 8px;
  }
  
  .btn-primary:hover {
    background: #2563eb;
  }
  
  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .backup-list {
    margin-top: 20px;
  }
  
  .backup-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    background: #0c1120;
    border: 1px solid #1f2a45;
    border-radius: 6px;
    margin-bottom: 8px;
  }
  
  .backup-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  
  .backup-name {
    font-size: 13px;
    color: #e0e7f5;
    font-family: monospace;
  }
  
  .backup-meta {
    font-size: 11px;
    color: #6b7a9a;
  }
  
  .encrypted {
    margin-left: 4px;
  }
  
  .backup-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  
  .btn-sm {
    background: #1c2944;
    border: 1px solid #2b3b60;
    color: #c7d4ec;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 11px;
  }
  
  .btn-sm.danger:hover {
    background: #3a1a2c;
    border-color: #ff5c8a;
  }
  
  .empty {
    text-align: center;
    padding: 16px;
    color: #6b7a9a;
    font-size: 12px;
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
</style>
