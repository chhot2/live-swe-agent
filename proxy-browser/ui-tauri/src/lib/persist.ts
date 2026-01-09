import { writable, get, type Writable } from 'svelte/store';

interface PersistOptions<T> {
  storage?: 'localStorage' | 'sessionStorage';
  serialize?: (value: T) => string;
  deserialize?: (value: string) => T;
}

export function persist<T>(
  store: Writable<T>,
  key: string,
  options: PersistOptions<T> = {}
): Writable<T> {
  const {
    storage = 'localStorage',
    serialize = JSON.stringify,
    deserialize = JSON.parse
  } = options;

  if (typeof window === 'undefined') {
    return store;
  }

  // Get initial value from storage
  try {
    const stored = window[storage].getItem(key);
    if (stored !== null) {
      store.set(deserialize(stored));
    }
  } catch (error) {
    console.warn(`Failed to load persisted value for ${key}:`, error);
  }

  // Subscribe to store changes and persist them
  store.subscribe(value => {
    try {
      window[storage].setItem(key, serialize(value));
    } catch (error) {
      console.warn(`Failed to persist value for ${key}:`, error);
    }
  });

  return store;
}
