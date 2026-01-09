import { writable, get } from 'svelte/store';
import { persist } from './persist';

/**
 * Theme options for the application
 */
export type ThemeMode = 'light' | 'dark' | 'system';

/**
 * Supported languages for i18n
 */
export type Language = 'en' | 'es' | 'fr' | 'de' | 'zh' | 'ja' | 'ko' | 'pt' | 'ru' | 'ar';

/**
 * Keyboard shortcut configuration
 */
export interface KeyboardShortcut {
  key: string;
  ctrl?: boolean;
  alt?: boolean;
  shift?: boolean;
  meta?: boolean;
}

/**
 * Customizable keyboard shortcuts
 */
export interface KeyboardShortcuts {
  newTab: KeyboardShortcut;
  closeTab: KeyboardShortcut;
  focusUrl: KeyboardShortcut;
  goBack: KeyboardShortcut;
  goForward: KeyboardShortcut;
  reload: KeyboardShortcut;
  switchTabNext: KeyboardShortcut;
  switchTabPrev: KeyboardShortcut;
  validateIp: KeyboardShortcut;
  rotateIp: KeyboardShortcut;
  toggleTheme: KeyboardShortcut;
  exportSession: KeyboardShortcut;
}

/**
 * Application settings interface
 */
interface AppSettings {
  // Display settings
  theme: ThemeMode;
  language: Language;
  
  // Performance settings
  virtualScrollEnabled: boolean;
  virtualScrollThreshold: number;
  connectionPoolEnabled: boolean;
  performanceMonitoring: boolean;
  autoCleanupEnabled: boolean;
  
  // Session settings
  saveSessionOnExit: boolean;
  restoreSessionOnStart: boolean;
  sessionAutoSaveInterval: number; // in minutes, 0 = disabled
  
  // Keyboard shortcuts
  keyboardShortcuts: KeyboardShortcuts;
  keyboardShortcutsEnabled: boolean;
}

/**
 * Default keyboard shortcuts
 */
const defaultKeyboardShortcuts: KeyboardShortcuts = {
  newTab: { key: 't', ctrl: true },
  closeTab: { key: 'w', ctrl: true },
  focusUrl: { key: 'l', ctrl: true },
  goBack: { key: 'ArrowLeft', alt: true },
  goForward: { key: 'ArrowRight', alt: true },
  reload: { key: 'r', ctrl: true },
  switchTabNext: { key: 'Tab', ctrl: true },
  switchTabPrev: { key: 'Tab', ctrl: true, shift: true },
  validateIp: { key: 'i', ctrl: true },
  rotateIp: { key: 'r', ctrl: true, alt: true },
  toggleTheme: { key: 'd', ctrl: true, shift: true },
  exportSession: { key: 's', ctrl: true, shift: true },
};

/**
 * Default application settings
 */
const defaultSettings: AppSettings = {
  // Display
  theme: 'system',
  language: 'en',
  
  // Performance
  virtualScrollEnabled: true,
  virtualScrollThreshold: 50,
  connectionPoolEnabled: true,
  performanceMonitoring: true,
  autoCleanupEnabled: true,
  
  // Session
  saveSessionOnExit: true,
  restoreSessionOnStart: true,
  sessionAutoSaveInterval: 5,
  
  // Keyboard shortcuts
  keyboardShortcuts: defaultKeyboardShortcuts,
  keyboardShortcutsEnabled: true,
};

// Create persistent settings store
export const settingsStore = persist(
  writable<AppSettings>(defaultSettings),
  'virtual-ip-browser-settings'
);

// Theme store with system detection
export const themeStore = writable<'light' | 'dark'>('light');

// Initialize theme based on settings and system preference
function initializeTheme() {
  const settings = get(settingsStore);
  if (settings.theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    themeStore.set(prefersDark ? 'dark' : 'light');
  } else {
    themeStore.set(settings.theme);
  }
}

// Listen for system theme changes
if (typeof window !== 'undefined') {
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    const settings = get(settingsStore);
    if (settings.theme === 'system') {
      themeStore.set(e.matches ? 'dark' : 'light');
    }
  });
  
  // Initialize on load
  initializeTheme();
}

// Update theme when settings change
settingsStore.subscribe(settings => {
  if (settings.theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    themeStore.set(prefersDark ? 'dark' : 'light');
  } else {
    themeStore.set(settings.theme);
  }
});

// Apply theme to document
themeStore.subscribe(theme => {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', theme);
    document.body.classList.remove('light-theme', 'dark-theme');
    document.body.classList.add(`${theme}-theme`);
  }
});

// Individual setting exports for convenience
export const virtualScrollEnabled = writable(get(settingsStore).virtualScrollEnabled);
export const virtualScrollThreshold = writable(get(settingsStore).virtualScrollThreshold);
export const connectionPoolEnabled = writable(get(settingsStore).connectionPoolEnabled);
export const performanceMonitoring = writable(get(settingsStore).performanceMonitoring);
export const autoCleanupEnabled = writable(get(settingsStore).autoCleanupEnabled);

// Sync individual stores with main settings store
settingsStore.subscribe(settings => {
  virtualScrollEnabled.set(settings.virtualScrollEnabled);
  virtualScrollThreshold.set(settings.virtualScrollThreshold);
  connectionPoolEnabled.set(settings.connectionPoolEnabled);
  performanceMonitoring.set(settings.performanceMonitoring);
  autoCleanupEnabled.set(settings.autoCleanupEnabled);
});

/**
 * Update a single setting
 */
export function updateSetting<K extends keyof AppSettings>(
  key: K,
  value: AppSettings[K]
): void {
  settingsStore.update(settings => ({
    ...settings,
    [key]: value
  }));
}

/**
 * Batch update settings
 */
export function updateSettings(newSettings: Partial<AppSettings>): void {
  settingsStore.update(settings => ({
    ...settings,
    ...newSettings
  }));
}

/**
 * Reset to defaults
 */
export function resetSettings(): void {
  settingsStore.set(defaultSettings);
}

/**
 * Get all settings
 */
export function getAllSettings(): AppSettings {
  return get(settingsStore);
}

/**
 * Toggle between light and dark theme
 */
export function toggleTheme(): void {
  const currentTheme = get(themeStore);
  const newTheme: ThemeMode = currentTheme === 'light' ? 'dark' : 'light';
  updateSetting('theme', newTheme);
}

/**
 * Set specific theme mode
 */
export function setTheme(theme: ThemeMode): void {
  updateSetting('theme', theme);
}

/**
 * Get current effective theme (resolves 'system' to actual theme)
 */
export function getEffectiveTheme(): 'light' | 'dark' {
  return get(themeStore);
}

/**
 * Update a keyboard shortcut
 */
export function updateKeyboardShortcut(
  action: keyof KeyboardShortcuts,
  shortcut: KeyboardShortcut
): void {
  settingsStore.update(settings => ({
    ...settings,
    keyboardShortcuts: {
      ...settings.keyboardShortcuts,
      [action]: shortcut
    }
  }));
}

/**
 * Reset keyboard shortcuts to defaults
 */
export function resetKeyboardShortcuts(): void {
  settingsStore.update(settings => ({
    ...settings,
    keyboardShortcuts: defaultKeyboardShortcuts
  }));
}

/**
 * Get keyboard shortcuts
 */
export function getKeyboardShortcuts(): KeyboardShortcuts {
  return get(settingsStore).keyboardShortcuts;
}

/**
 * Format a keyboard shortcut for display
 */
export function formatShortcut(shortcut: KeyboardShortcut): string {
  const parts: string[] = [];
  if (shortcut.ctrl) parts.push('Ctrl');
  if (shortcut.alt) parts.push('Alt');
  if (shortcut.shift) parts.push('Shift');
  if (shortcut.meta) parts.push('⌘');
  parts.push(shortcut.key);
  return parts.join('+');
}
