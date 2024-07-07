import type { Drawable } from '$lib/drawing';
import { XY } from '$lib/point';
import { writable } from 'svelte/store';

export type CanvasInterface = {
  centerOn: (loc:XY, zoom:number|undefined) => void,
  draw: () => void,
};

function createInterface() {
  const { subscribe, set, update  } = writable<CanvasInterface>({
    centerOn: (loc: XY, zoom:number|undefined) => console.log(`Centering on ${loc} at zoom ${zoom}.`),
    draw: () => console.log("Drawing."),
  });

  return {
    subscribe,
    set: (value: CanvasInterface) => set(value),
  };
}

export const canvasInterface = createInterface();


export const canvasClick = writable<XY>(new XY(0, 0));


export type CanvasSettings = {
  tracking: boolean, // Zoom to the center point instead of the cursor.
  redraw: boolean, // Immediately redraw the canvas after moving the camera.  Enable when paused.
};

function createCanvasSettings() {
  const { subscribe, set, update  } = writable<CanvasSettings>({
    tracking: false,
    redraw: false,
  });

  return {
    subscribe,
    set: (value: CanvasSettings) => set(value),
    update: (value: Partial<CanvasSettings>) => update((settings) => ({...settings, ...value})),
  };
}

export const canvasSettings = createCanvasSettings();


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