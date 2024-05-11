import type { Drawable } from '$lib/drawing';
import type { XY } from '$lib/point';
import { writable } from 'svelte/store';

export type CanvasInterface = {
  centerOn: (loc:XY, zoom:number|undefined, redraw:boolean) => void,
  draw: () => void,
};

function createInterface() {
  const { subscribe, set, update  } = writable<CanvasInterface>(undefined);

  return {
    subscribe,
    set: (value: CanvasInterface) => set(value),
  };
}

export const canvasInterface = createInterface();


export type DrawEntity = {

}

export type DrawBuffer = Drawable[];

function createDrawBuffer() {
  const { subscribe, set, update } = writable<DrawBuffer>([]);

  return {
    subscribe,
    set: (value: Drawable[]) => set(value),
    add: (value: Drawable) => update((buffer) => [...buffer, value]),
    clear: () => set([])
  };
}

export const drawBuffer = createDrawBuffer();