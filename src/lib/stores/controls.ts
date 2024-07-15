import { writable } from "svelte/store";

export type ControlsInterface = {
  home: () => void,
  step: () => Promise<void>,
  play: () => void,
  pause: () => void,
  fastforward: () => void,
  toggleTracking: () => void,
  redraw: () => Promise<void>,
};

function createInterface() {
  const { subscribe, set, update } = writable<ControlsInterface>({
    home: () => console.log("home"),
    step: async () => console.log("step"),
    play: () => console.log("play"),
    pause: () => console.log("pause"),
    fastforward: () => console.log("fastforward"),
    toggleTracking: () => console.log("toggleTracking"),
    redraw: async () => console.log("redraw"),
  });

  return {
    subscribe,
    set: (value: ControlsInterface) => set(value),
  };
}

export const controlsInterface = createInterface();