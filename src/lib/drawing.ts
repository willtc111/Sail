import { XY } from "./point";
import type { Shape } from "./types";

const BORDER_WIDTH = 1;

export interface Drawable {
  draw(ctx: CanvasRenderingContext2D): void;
}

export class Axis implements Drawable {
  dimensions: XY;
  steps: XY;
  grid: boolean;
  axesColor: string;
  stepColor: string;

  constructor(dimensions: XY, steps: XY, grid: boolean, axesColor: string = '#ffffff', stepColor: string = '#111144') {
    this.dimensions = dimensions;
    this.steps = steps;
    this.grid = grid;
    this.axesColor = axesColor;
    this.stepColor = stepColor;
  }

  draw(ctx: CanvasRenderingContext2D) {
    // Axis lines
    ctx.beginPath();
    ctx.moveTo(this.dimensions.x, 0);
    ctx.lineTo(-this.dimensions.x, 0);
    ctx.moveTo(0, this.dimensions.x);
    ctx.lineTo(0, -this.dimensions.x);
    ctx.strokeStyle=this.axesColor;
    ctx.stroke();

    // Tick marks
    let minX = this.grid ? -this.dimensions.x : 0;
    let maxX = this.grid ? this.dimensions.x : 2;
    let minY = this.grid ? -this.dimensions.y : 0;
    let maxY = this.grid ? this.dimensions.y : 2;
    ctx.beginPath();
    for (let x = this.steps.x; x <= this.dimensions.x; x += this.steps.x) {
      ctx.moveTo(x, minY);
      ctx.lineTo(x, maxY);
      ctx.moveTo(-x, minY);
      ctx.lineTo(-x, maxY);
    }
    for (let y = this.steps.y; y <= this.dimensions.y; y += this.steps.y) {
      ctx.moveTo(minX, y);
      ctx.lineTo(maxX, y);
      ctx.moveTo(minX, -y);
      ctx.lineTo(maxX, -y);
    }
    ctx.lineWidth = 1;
    ctx.strokeStyle=this.stepColor;
    ctx.stroke();
  }
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
      ctx.lineWidth = this.width;
      ctx.strokeStyle=this.stroke;
      ctx.stroke();
    }
  }
}

export class Arrow extends Line {
  head_left: XY;
  head_right: XY;
  head_size: number;

  constructor(start: XY, end: XY, width: number, head_size:number, stroke:string) {
    super(start, end, width, stroke);

    this.head_size = head_size;
    let dy = this.end.y - this.start.y;
    let dx = this.end.x - this.start.x;
    let line_angle = Math.atan2(dy, dx);
    let head_angle = Math.PI / 8;

    this.head_left = new XY(
      this.end.x - head_size * Math.cos(line_angle - head_angle),
      this.end.y - head_size * Math.sin(line_angle - head_angle)
    );
    this.head_right = new XY(
      this.end.x - head_size * Math.cos(line_angle + head_angle),
      this.end.y - head_size * Math.sin(line_angle + head_angle)
    );
  }

  draw(ctx: CanvasRenderingContext2D) {
    if (this.stroke == undefined) {
      return;
    }
    ctx.beginPath();
    // Line
    ctx.moveTo(this.start.x, this.start.y);
    ctx.lineTo(this.end.x, this.end.y);
    // Head
    ctx.moveTo(this.head_left.x, this.head_left.y);
    ctx.lineTo(this.end.x, this.end.y);
    ctx.lineTo(this.head_right.x, this.head_right.y);

    ctx.lineWidth = this.width;
    ctx.strokeStyle = this.stroke;
    ctx.stroke();
  }
}

export class Polygon implements Drawable {
  points: XY[];
  fill: string|undefined;
  stroke: string|undefined;

  constructor(points:XY[], fill:string|undefined=undefined, stroke:string|undefined=undefined) {
    this.points = points;
    this.fill = fill;
    this.stroke = stroke;
  }

  draw(ctx: CanvasRenderingContext2D) {
    if (this.stroke == undefined && this.fill == undefined) {
      return;
    }
    ctx.beginPath();
    for (let i = 0; i < this.points.length; i++) {
      if (i == 0) {
        ctx.moveTo(this.points[i].x, this.points[i].y);
      } else {
        ctx.lineTo(this.points[i].x, this.points[i].y);
      }
    }
    ctx.closePath();
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

export class Polyline implements Drawable {
  points: XY[];
  width: number;
  stroke: string;

  constructor(points:XY[], width:number, stroke:string) {
    this.points = points;
    this.width = width;
    this.stroke = stroke;
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.beginPath();
    for (let i = 0; i < this.points.length; i++) {
      if (i == 0) {
        ctx.moveTo(this.points[i].x, this.points[i].y);
      } else {
        ctx.lineTo(this.points[i].x, this.points[i].y);
      }
    }
    ctx.lineWidth = this.width;
    ctx.strokeStyle = this.stroke;
    ctx.stroke();
  }
}

export class Graph implements Drawable {
  points: Point[];
  lines: Line[];

  constructor(points: Point[], lines: Line[]) {
    this.points = points;
    this.lines = lines;
  }

  draw(ctx: CanvasRenderingContext2D) {
    this.points.forEach(p => p.draw(ctx));
    this.lines.forEach(l => l.draw(ctx));
  }
}
export function shapeToGraph(shape: Shape): Graph {
  let points: Point[] = [];
  shape.vertices.forEach(v => points.push(
    new Point(
      new XY(v.x, v.y),
      1, 
      'green',
      'green'
    )
  ));

  let lines: Line[] = [];
  shape.edges.forEach(e => {
    let from = points[e[0]];
    let to = points[e[1]];
    lines.push(
      new Line(
        from.loc,
        to.loc,
        2,
        'green'
      )
    )
  }
  );

  return new Graph(points, lines);
}
type PrePolygon = {
  points: XY[]
};
export class Ship implements Drawable {
  center: XY;
  hull: Polygon;
  sails: Polygon[];
  rudder: Polygon;

  constructor(center: XY, hull: PrePolygon, sails: PrePolygon[], rudder: PrePolygon, hull_color: string = 'brown', sail_color: string = 'white') {
    this.center = center;
    this.hull = new Polygon(hull.points, hull_color);
    this.sails = sails.map(s => new Polygon(s.points, sail_color));
    this.rudder = new Polygon(rudder.points, hull_color);
  }

  draw(ctx: CanvasRenderingContext2D) {
    // Draw from the bottom up
    this.rudder.draw(ctx);
    this.hull.draw(ctx);
    this.sails.forEach(s => s.draw(ctx));
  }
}