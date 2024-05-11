import type { XY } from "./point";

const BORDER_WIDTH = 1;

export interface Drawable {
  draw(ctx: CanvasRenderingContext2D): void;
}

export class Rectangle implements Drawable {
  loc: XY;
  dim: XY;
  fill: string|undefined;
  stroke: string|undefined;

  constructor(loc: XY, dim: XY, fill:string|undefined=undefined, stroke:string|undefined=undefined) {
    this.loc = loc;
    this.dim = dim;
    this.fill = fill;
    this.stroke = stroke;
  }

  draw(ctx: CanvasRenderingContext2D) {
    if (this.fill == undefined && this.stroke == undefined) {
      return;
    }
    ctx.beginPath();
    ctx.rect(this.loc.x, this.loc.y, this.dim.x, this.dim.y);
    if (this.fill != undefined) {
      ctx.fillStyle=this.fill;
      ctx.fill();
    }
    if (this.stroke != undefined) {
      ctx.lineWidth = BORDER_WIDTH;
      ctx.strokeStyle=this.stroke;
      ctx.stroke();
    }
  }
}

export class Point implements Drawable {
  loc: XY;
  radius: number
  fill: string;
  stroke: string;

  constructor(loc: XY, radius: number, fill:string, stroke:string) {
    this.loc = loc;
    this.radius = radius;
    this.fill = fill;
    this.stroke = stroke;
  }

  draw(ctx: CanvasRenderingContext2D) {
    if (this.fill == undefined && this.stroke == undefined) {
      return;
    }
    ctx.beginPath();
    ctx.arc(this.loc.x, this.loc.y, this.radius, 0, 2 * Math.PI);
    if (this.fill != undefined) {
      ctx.fillStyle=this.fill;
      ctx.fill();
    }
    if (this.stroke != undefined) {
      ctx.lineWidth = BORDER_WIDTH;
      ctx.strokeStyle=this.stroke;
      ctx.stroke();
    }
  }
}

export class Line implements Drawable {
  start: XY;
  end: XY;
  width: number
  stroke: string;

  constructor(start: XY, end: XY, width: number, stroke:string) {
    this.start = start;
    this.end = end;
    this.width = width;
    this.stroke = stroke;
  }

  draw(ctx: CanvasRenderingContext2D) {
    if (this.stroke == undefined) {
      return;
    }
    ctx.beginPath();
    ctx.moveTo(this.start.x, this.start.y);
    ctx.lineTo(this.end.x, this.end.y);
    if (this.stroke != undefined) {
      ctx.lineWidth = BORDER_WIDTH;
      ctx.strokeStyle=this.stroke;
      ctx.stroke();
    }
  }
}