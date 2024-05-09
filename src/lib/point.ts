
export type xy = { x: number, y: number };

export function add(a:xy, b:xy): xy {
  return {
    x: a.x + b.x,
    y: a.y + b.y
  };
}

export function sub(a:xy, b:xy): xy {
  return {
    x: a.x - b.x,
    y: a.y - b.y
  };
}

export function mul(a:xy, s: number): xy {
  return {
    x: a.x  * s,
    y: a.y  * s
  };
}

export function div(a:xy, s: number): xy {
  return {
    x: a.x  / s,
    y: a.y  / s
  };
}
