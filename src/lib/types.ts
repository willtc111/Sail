// Types returned when invoking Rust functions

import type { XY } from "./point"

export type Shape = {
  vertices: XY[],
  edges: [number, number][]
};

export type ShipData = {
  loc: XY,
  vel: XY,
  rot_vel: number,
  heading: number,
  sail_angle: number,
  mainsheet_length: number,
  rudder_angle: number,
}