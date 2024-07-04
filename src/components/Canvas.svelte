<script lang="ts">
  import { Rectangle } from "$lib/drawing";
  import { XY} from "$lib/point";
  import { canvasInterface, canvasSettings, drawBuffer } from "$lib/stores/canvasInterface";

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;
  $: if (canvas) {
    ctx = canvas.getContext('2d', { alpha: false });
    centerOn(new XY(0,0), 1);
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

  canvasInterface.set({
    centerOn: centerOn,
    draw: draw,
  });

  function centerOn(loc:XY, zoom:number|undefined=undefined) {
    if (zoom != undefined) {
      camera.zoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom))
    }
    camera.offset = offsetFromCenterCoordinate(loc);
    if ($canvasSettings.redraw) {
      draw();
    }
  }

  function draw() {
    if (ctx == null) { return; }

    ctx.reset();
    // Negate Y zoom to flip canvas to be "normal" coordinates
    ctx.setTransform(camera.zoom, 0, 0, -camera.zoom, camera.offset.x, camera.offset.y);

    $drawBuffer.forEach((entity) => {
      entity.draw(ctx!)
    });
  }

  const zoomMagnitude = 1.1;
  const MIN_ZOOM = 0.25;
  const MAX_ZOOM = 30;

  let camera = {
    offset: new XY(0.0, 0.0),
    zoom: 1.0,
  };

  let dragging: boolean = false;
  let dragStart: XY = new XY();
  let clickStart: XY | undefined;

  function onMouseDown(event: MouseEvent) {
    dragging = true;
    let loc_client = getEventLocation(event);
    dragStart = loc_client.sub(camera.offset);
    clickStart = loc_client;
  }

  function onMouseMove(event: MouseEvent) {
    let loc_client = getEventLocation(event);

    if (dragging) {
      if ($canvasSettings.tracking) {
        // This is the first mouse movement to break the tracking
        // so reset dragStart to ensure a smooth transition.
        dragStart = loc_client.sub(camera.offset);
        $canvasSettings.tracking = false;
      }
      camera.offset = loc_client.sub(dragStart);
      if ($canvasSettings.redraw) {
        draw();
      }
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
    }
    clickStart = undefined;
  }

  function onWheel(event: WheelEvent) {
    let zoomScale = event.deltaY > 0 ? 1/zoomMagnitude : zoomMagnitude
    let newZoom = camera.zoom * zoomScale;
    if (newZoom > MAX_ZOOM || newZoom < MIN_ZOOM) {
      return;
    }
    let loc_client = getEventLocation(event);
    let loc_canvas = toCanvasFromClient(loc_client);
    camera.zoom = newZoom;
    if ($canvasSettings.tracking) {
      let center = new XY(width/2, height/2);
      camera.offset = center.sub(center.sub(camera.offset).scale(zoomScale));
    } else {
      camera.offset = loc_canvas.sub(loc_canvas.sub(camera.offset).scale(zoomScale));
    }
    if (dragging) {
      dragStart = loc_client.sub(camera.offset);
    }
    if ($canvasSettings.redraw) {
      draw();
    }
  }

  function getEventLocation(event:MouseEvent): XY {
    return new XY(event.clientX, event.clientY);
  }

  function offsetFromCenterCoordinate(center:XY): XY {
    let dimensions = new XY(width, height);
    return center.scale(-1*camera.zoom).flipY().add(dimensions.scale(1/2));
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
    return loc.sub(camera.offset).scale(1/camera.zoom).flipY();
  }

  function toCanvasFromDisplay(loc:XY): XY {
    return loc.flipY().scale(camera.zoom).add(camera.offset);
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