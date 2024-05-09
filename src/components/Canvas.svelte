<script lang="ts">
  import { type xy, div, mul, sub } from "$lib/point";

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
    console.log("home")
    offset = {x:0.0, y:0.0};
    zoom = 1.0;
  }

  let test = 0;
  export const draw = () => {
    if (ctx == null) { return; }

    ctx.reset();
    ctx.setTransform(zoom, 0, 0, zoom, offset.x, offset.y);

    // background
    ctx.fillStyle='white';
    ctx.fillRect(0,0,width,height);

    // border
    drawRect(
      10, 10,
      width-20, height-20,
      undefined, '#000000'
    );

    // debug
    test++;
    drawRect(
      50 + 35*Math.cos(test/10),
      50 + 35*Math.sin(test/10),
      20, 20,
      'purple', undefined
    );

    // debug coordinate transform
    drawRect(-10,-10,20,20,'red'); // center
    drawRect(10, 10, 5, 5,'red'); // quadrant 1
    drawRect(-10, 10, -5, 5,'red'); // quadrant 2
    drawRect(-10, -10, -5, -5,'red'); // quadrant 3
    drawRect(10, -10, 5, -5,'red'); // quadrant 4
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

  let size_ratio = 1; //image_size / canvas_size;
  const zoomMagnitude = 1.1;
  const MIN_ZOOM = 0.25;
  const MAX_ZOOM = 30;

  let offset: xy = {x: 0.0, y: 0.0};
  let zoom = 1.0;

  let dragging: boolean = false;
  let dragStart: xy = { x: 0, y: 0 };
  let clickStart: xy | undefined;

  function onMouseDown(event: MouseEvent) {
    dragging = true;
    let loc_client = getEventLocation(event);
    dragStart = sub(loc_client, offset);
    clickStart = loc_client;
  }

  function onMouseMove(event: MouseEvent) {
    let loc_client = getEventLocation(event);

    if (dragging) {
      offset = sub(loc_client, dragStart);
      draw();
    }
  }

  function onMouseUp(event: MouseEvent) {
    dragging = false;
    let loc_client = getEventLocation(event);

    if (clickStart != undefined && clickStart.x == loc_client.x && clickStart.y == loc_client.y) {
      // Mouse didn't move, consider this a click
    }
    clickStart = undefined;
  }

  function onWheel(event: WheelEvent) {
    let scale = event.deltaY > 0 ? 1/zoomMagnitude : zoomMagnitude
    let newZoom = zoom * scale;
    if (newZoom > MAX_ZOOM || newZoom < MIN_ZOOM) {
      return;
    }
    let loc_client = getEventLocation(event);
    let loc_canvas = toCanvasFromClient(loc_client);
    zoom = newZoom;
    offset = sub(loc_canvas, mul(sub(loc_canvas, offset), scale));
    if (dragging) {
      dragStart = sub(loc_client, offset);
    }
    draw();
  }

  function getEventLocation(event:MouseEvent): xy {
    return { x: event.clientX, y: event.clientY };
  }

  //
  // Coordinate conversions
  //
  // client space: coordinates within the window
  // canvas space: coordinates within the canvas element
  // display space: coordinates within the display (zoom/pan)
  // source space: coordinates within the source material
  //

  function toCanvasFromClient(loc:xy): xy {
    let rect = canvas.getBoundingClientRect();
    return sub(loc, {x: rect.left, y: rect.top});
  }

  function toDisplayFromCanvas(loc:xy): xy {
    return div(sub(loc, offset), zoom);
  }

  function toSourceFromDisplay(loc:xy): xy {
    return mul(loc, size_ratio);
  }

  function toDisplayFromSource(loc:xy): xy {
    return div(loc, size_ratio);
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