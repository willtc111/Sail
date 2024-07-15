import { writable } from "svelte/store";

function createStore() {
  const { subscribe, set, update } = writable<number>(0.0);

  return {
    subscribe,
    set,
  };
}

export const simulationStep = createStore();