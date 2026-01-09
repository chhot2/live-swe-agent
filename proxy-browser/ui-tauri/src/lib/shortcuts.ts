import { createKeyboardShortcut } from './utils';
import type { Tab } from './types';

export interface ShortcutHandlers {
  newTab: () => void;
  closeTab: () => void;
  focusUrl: () => void;
  goBack: () => void;
  goForward: () => void;
  reload: () => void;
  switchTab: (direction: 'next' | 'prev') => void;
  validateIp: () => void;
  rotateIp: () => void;
}

export class KeyboardShortcutManager {
  private handlers: ShortcutHandlers;
  private boundHandlers: Array<(e: KeyboardEvent) => void> = [];
  private enabled = false;

  constructor(handlers: ShortcutHandlers) {
    this.handlers = handlers;
  }

  enable() {
    if (this.enabled) return;
    
    const shortcuts = [
      // Tab management
      { key: 't', ctrl: true, handler: this.handlers.newTab },
      { key: 'w', ctrl: true, handler: this.handlers.closeTab },
      { key: 'Tab', ctrl: true, handler: () => this.handlers.switchTab('next') },
      { key: 'Tab', ctrl: true, shift: true, handler: () => this.handlers.switchTab('prev') },
      
      // Navigation
      { key: 'l', ctrl: true, handler: this.handlers.focusUrl },
      { key: 'ArrowLeft', alt: true, handler: this.handlers.goBack },
      { key: 'ArrowRight', alt: true, handler: this.handlers.goForward },
      { key: 'r', ctrl: true, handler: this.handlers.reload },
      { key: 'F5', handler: this.handlers.reload },
      
      // IP actions
      { key: 'i', ctrl: true, handler: this.handlers.validateIp },
      { key: 'r', ctrl: true, alt: true, handler: this.handlers.rotateIp },
      
      // Quick actions
      { key: '1', alt: true, handler: () => this.switchToTab(0) },
      { key: '2', alt: true, handler: () => this.switchToTab(1) },
      { key: '3', alt: true, handler: () => this.switchToTab(2) },
      { key: '4', alt: true, handler: () => this.switchToTab(3) },
      { key: '5', alt: true, handler: () => this.switchToTab(4) },
      { key: '6', alt: true, handler: () => this.switchToTab(5) },
      { key: '7', alt: true, handler: () => this.switchToTab(6) },
      { key: '8', alt: true, handler: () => this.switchToTab(7) },
      { key: '9', alt: true, handler: () => this.switchToTab(8) },
    ];

    this.boundHandlers = shortcuts.map(shortcut => {
      const handler = createKeyboardShortcut(
        shortcut.key,
        shortcut.handler,
        { ctrl: shortcut.ctrl || false, alt: shortcut.alt || false, shift: shortcut.shift || false }
      );
      document.addEventListener('keydown', handler);
      return handler;
    });

    this.enabled = true;
  }

  disable() {
    if (!this.enabled) return;
    
    this.boundHandlers.forEach(handler => {
      document.removeEventListener('keydown', handler);
    });
    this.boundHandlers = [];
    this.enabled = false;
  }

  private switchToTab(index: number) {
    // This will be handled by the parent component
    const event = new CustomEvent('switchToTab', { detail: { index } });
    document.dispatchEvent(event);
  }
}

// Helper to check if element is an input that should handle shortcuts
export function shouldIgnoreShortcut(element: EventTarget | null): boolean {
  if (!element) return false;
  
  const el = element as HTMLElement;
  const tagName = el.tagName.toLowerCase();
  
  // Ignore shortcuts when typing in inputs, textareas, or contenteditable
  if (tagName === 'input' || tagName === 'textarea' || el.contentEditable === 'true') {
    // Allow Ctrl+A, Ctrl+C, Ctrl+V, Ctrl+X, Ctrl+Z
    const activeElement = document.activeElement as HTMLInputElement;
    if (activeElement && (activeElement.type === 'text' || activeElement.type === 'url' || activeElement.type === 'search')) {
      return true;
    }
  }
  
  return false;
}

// Enhanced keyboard shortcut that respects input focus
export function createSmartKeyboardShortcut(
  key: string,
  callback: () => void,
  options: { ctrl?: boolean; alt?: boolean; shift?: boolean } = {}
) {
  return function(event: KeyboardEvent) {
    // Don't trigger shortcuts when typing in inputs
    if (shouldIgnoreShortcut(event.target)) {
      // Allow specific shortcuts even in inputs
      const allowedShortcuts = ['t', 'w', 'l', 'Tab'];
      if (!allowedShortcuts.includes(key)) {
        return;
      }
    }
    
    const ctrlMatch = options.ctrl ? event.ctrlKey || event.metaKey : !event.ctrlKey && !event.metaKey;
    const altMatch = options.alt ? event.altKey : !event.altKey;
    const shiftMatch = options.shift ? event.shiftKey : !event.shiftKey;
    
    if (event.key === key && ctrlMatch && altMatch && shiftMatch) {
      event.preventDefault();
      callback();
    }
  };
}
