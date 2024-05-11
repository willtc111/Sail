import { writable } from 'svelte/store';

function createSelection() {
  const { subscribe, set, update  } = writable<number|undefined>(undefined);

  return {
    subscribe,
    set: (value: number) => set(value),
    clear: () => set(undefined)
  };
}

export const selection = createSelection();
