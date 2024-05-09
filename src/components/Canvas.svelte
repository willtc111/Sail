<script lang="ts">
  import { XY} from "$lib/point";

  export let dishWidth;
  export let dishHeight;

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;
  $: if (canvas) {
    ctx = canvas.getContext('2d');
  }

  // Window dimensions (for responsive resizing)
  let innerWidth: number;
  let innerHeight: number;
  // Canvas dimensions
  let width = 100;
  let height = 200;
  $: if (innerHeight && innerWidth) {
    width = canvas?.clientWidth ?? 100;
    height = canvas?.clientHeight ?? 200;
    draw();
  }

  export const home = () => {
    zoom = 1.0;
    centerOn(new XY(dishWidth, dishHeight).scale(1/2));
  }

  export const centerOn = (loc:XY) => {
    offset = offsetFromCenterCoordinate(loc);
    draw();
  }

  let test = 0;
  export const draw = () => {
    if (ctx == null) { return; }

    ctx.reset();
    ctx.setTransform(zoom, 0, 0, zoom, offset.x, offset.y);

    // background
    ctx.fillStyle = 'white';
    ctx.fillRect(0, 0, dishWidth, dishHeight);

    // border
    drawRect(
      10, 10,
      width-20, height-20,
      undefined, '#000000'
    );
    drawRect(
      0, 0,
      dishWidth, dishHeight,
      undefined, '#0000ff'
    );

    // debug
    test++;
    drawRect(
      (dishWidth/2 - 10) + 35*Math.cos(test/10),
      (dishHeight/2 - 10) + 35*Math.sin(test/10),
      20, 20,
      'purple', undefined
    );

    drawRect(
      dishWidth/2, dishHeight/2,
      5, 5,
      'purple', undefined
    );
    drawRect(
      dishWidth/2, dishHeight/2,
      -5, -5,
      'purple', undefined
    );

    // debug coordinate transform
    drawRect(-10, -10, 20, 20, 'red'); // center
    drawRect(0, 0, 1, 1, 'green'); // center
    drawRect(10, 10, 5, 5, 'red'); // quadrant 1
    drawRect(-10, 10, -5, 5, 'red'); // quadrant 2
    drawRect(-10, -10, -5, -5, 'red'); // quadrant 3
    drawRect(10, -10, 5, -5, 'red'); // quadrant 4
  }

  function drawRect(x:number, y:number, w:number, h:number, fill:string|undefined=undefined, stroke:string|undefined=undefined) {
    if (!ctx || (fill == undefined && stroke == undefined)) { console.log("nope"); return; }
    ctx.beginPath();
    ctx.rect(x, y, w, h);
    if (fill != undefined) {
      ctx.fillStyle=fill;
      ctx.fill();
    }
    if (stroke != undefined) {
      ctx.strokeStyle=stroke;
      ctx.stroke();
    }
  }

  const zoomMagnitude = 1.1;
  const MIN_ZOOM = 0.25;
  const MAX_ZOOM = 30;

  let offset: XY = new XY(0.0, 0.0);
  let zoom = 1.0;

  let dragging: boolean = false;
  let dragStart: XY = new XY();
  let clickStart: XY | undefined;

  function onMouseDown(event: MouseEvent) {
    dragging = true;
    let loc_client = getEventLocation(event);
    dragStart = loc_client.sub(offset);
    clickStart = loc_client;
  }

  function onMouseMove(event: MouseEvent) {
    let loc_client = getEventLocation(event);

    if (dragging) {
      offset = loc_client.sub(dragStart);
      draw();
    }
  }

  function onMouseUp(event: MouseEvent) {
    dragging = false;
    let loc_client = getEventLocation(event);

    if (clickStart != undefined && clickStart.x == loc_client.x && clickStart.y == loc_client.y) {
      // Consider this a click
      let loc_canvas = toCanvasFromClient(loc_client);
      let loc_display = toDisplayFromCanvas(loc_canvas);
      // debug
      console.log(`click at disply ${loc_display.toStringFixed(1)}`);
      centerOn(loc_display);
    }
    clickStart = undefined;
  }

  function onWheel(event: WheelEvent) {
    let zoomScale = event.deltaY > 0 ? 1/zoomMagnitude : zoomMagnitude
    let newZoom = zoom * zoomScale;
    if (newZoom > MAX_ZOOM || newZoom < MIN_ZOOM) {
      return;
    }
    let loc_client = getEventLocation(event);
    let loc_canvas = toCanvasFromClient(loc_client);
    zoom = newZoom;
    offset = loc_canvas.sub(loc_canvas.sub(offset).scale(zoomScale));
    if (dragging) {
      dragStart = loc_client.sub(offset);
    }
    draw();
  }

  function getEventLocation(event:MouseEvent): XY {
    return new XY(event.clientX, event.clientY);
  }

  function offsetFromCenterCoordinate(center:XY): XY {
    console.log(`Calculating offset for center point ${center}`);
    let dimensions = new XY(width, height);
    return center.scale(-1*zoom).add(dimensions.scale(1/2));
  }

  //
  // Coordinate conversions
  //
  // client space: coordinates within the window
  // canvas space: coordinates within the canvas element
  // display space: coordinates within the display (zoom/pan)
  //

  function toCanvasFromClient(loc:XY): XY {
    let rect = canvas.getBoundingClientRect();
    return loc.sub({x: rect.left, y: rect.top});
  }

  function toDisplayFromCanvas(loc:XY): XY {
    return loc.sub(offset).scale(1/zoom);
  }

  function toCanvasFromDisplay(loc:XY): XY {
    return loc.scale(zoom).add(offset);
  }
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<div class="w-full h-full border border-surface-200-700-token relative rounded-container-token">
  {#key width+height}
    <canvas
      bind:this={canvas}
      {width}
      {height}
      class="w-full h-full rounded-container-token"
      on:wheel|stopPropagation|preventDefault={onWheel}
      on:mousedown|stopPropagation|preventDefault={onMouseDown}
      on:mousemove={onMouseMove}
      on:mouseup={onMouseUp}
      on:mouseleave={onMouseUp}
    />
  {/key}
</div>