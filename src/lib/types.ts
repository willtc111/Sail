// Types returned when invoking Rust functions

import type { XY } from "./point"

export type Shape = {
  vertices: XY[],
  edges: [number, number][]
};

export type Ship = {
  center: XY,
  heading: number,
  rudder_angle: number,
  sail_angle: number,
};