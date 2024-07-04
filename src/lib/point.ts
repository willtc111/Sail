
export type xy = { x: number, y: number };

export class XY {
  x: number;
  y: number;

  constructor(x:number=0.0, y:number=0.0) {
    this.x = x;
    this.y = y;
  }

  static from(xy: {x: number, y: number}): XY {
    return new XY(xy.x, xy.y);
  }

  add(other:xy): XY {
    return new XY(
      this.x + other.x,
      this.y + other.y
    );
  }

  sub(other:xy): XY {
    return new XY(
      this.x - other.x,
      this.y - other.y
    );
  }

  mul(other:xy): XY {
    return new XY(
      this.x  * other.x,
      this.y  * other.y
    );
  }

  div(other:xy): XY {
    return new XY(
      this.x  / other.x,
      this.y  / other.y
    );
  }

  scale(s: number): XY {
    return new XY(
      this.x  * s,
      this.y  * s
    );
  }

  flipY(): XY {
    return new XY(
      this.x,
      this.y * -1
    );
  }

  toString() {
    return `(${this.x}, ${this.y})`;
  }

  toStringFixed(digits:number) {
    return `(${this.x.toFixed(digits)}, ${this.y.toFixed(digits)})`;
  }
}