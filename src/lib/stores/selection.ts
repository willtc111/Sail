import { writable } from 'svelte/store';

function createSelection() {
  const { subscribe, set, update  } = writable<number|null>(null);

  return {
    subscribe,
    set: (value: number) => set(value),
    clear: () => set(null)
  };
}

export const selection = createSelection();
