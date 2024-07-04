<script lang="ts">
  import { Axis, Rectangle, Ship } from "$lib/drawing";
  import { RollingAverage } from "$lib/performance";
  import { XY } from "$lib/point";
  import { canvasInterface, canvasSettings, drawBuffer } from "$lib/stores/canvasInterface";
  import { invoke } from "@tauri-apps/api";

  export let dishWidth: number;
  export let dishHeight: number;

  let {draw, centerOn} = $canvasInterface;

  let loopTimeoutId: NodeJS.Timeout | undefined;
  function pause() {
    stopLoop();
    playing = false;
    fastforwarding = false;
  }

  let mspf = new RollingAverage();
  let average: string | number = "---";
  let lastTime = Date.now();

  let stepCount = 0;
  async function stepUpdate() {
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    stepCount = await invoke('step_simulation') as number;
    await stepDraw();
    let curTime = Date.now();
    let elapsed = curTime - lastTime;
    lastTime = curTime;
    console.log(`elapsed: ${elapsed}`);
    mspf.add(elapsed);
    average = mspf.get() ?? "---";
  }

  stepDraw();

  let trackLocation: XY|undefined = undefined;
  async function stepDraw() {
    let halfWidth = dishWidth / 2;
    let halfHeight = dishHeight / 2;
    drawBuffer.set([
      new Axis(
        new XY(halfWidth, halfHeight),
        new XY(10, 10),
        true
      ),
      new Rectangle(
        new XY(-halfWidth-1, -halfHeight-1),
        new XY(dishWidth+2, dishHeight+2),
        undefined, '#0000ff'
      )
    ]);

    let ships = await invoke('get_population') as any[];
    ships = ships.map(s => new Ship(
      XY.from(s.center),
      s.hull.map((xy: { x: number; y: number; }) => XY.from(xy)),
      s.sail.map((xy: { x: number; y: number; }) => XY.from(xy)),
      s.rudder.map((xy: { x: number; y: number; }) => XY.from(xy)),
      'brown', 'white'));
    ships.forEach(d => drawBuffer.add(d));

    trackLocation = ships[0].center;
    if ($canvasSettings.tracking) {
      centerOn(trackLocation!, undefined);
    }
    draw();
    console.log("Drew");
  }

  function loopStep() {
    stepUpdate();
    loopTimeoutId = setTimeout(
      () => loopStep(),
      fastforwarding ? 0 : 1000/60
    );
  }

  function stopLoop() {
    if (loopTimeoutId != undefined) {
      clearTimeout(loopTimeoutId);
      loopTimeoutId = undefined;
    }
  }

  let playing = false;
  $: $canvasSettings.redraw = !playing;

  function play() {
    stopLoop();
    playing = true;
    mspf.clear();
    loopStep();
  }

  let fastforwarding = false;
  function fastforward() {
    fastforwarding = !fastforwarding;
  }

  function home() {
    $canvasSettings.tracking = false;
    centerOn(new XY(0.0, 0.0), 1.0);
  }

  function trackSelection() {
    if (trackLocation) {
      $canvasSettings.tracking = !$canvasSettings.tracking;
      if ($canvasSettings.tracking) {
        centerOn(trackLocation, undefined);
      }
    }
  }
</script>

<div class="p-1 flex gap-2 justify-center">
  <button
    class="btn w-8 h-6 variant-filled-primary"
    title="Reset View"
    on:click={home}
  >
    <i class="fa fa-house" />
  </button>
  <button
    class="btn w-8 h-6 variant-filled-primary"
    title="Step"
    on:click={stepUpdate}
    disabled={playing}
  >
    <i class="fa-solid fa-forward-step" />
  </button>
  {#if playing}
    <button
      class="btn w-8 h-6 variant-filled-primary"
      title="Pause"
      on:click={pause}
      disabled={!playing}
    >
      <i class="fa fa-pause" />
    </button>
  {:else}
    <button
      class="btn w-8 h-6 variant-filled-primary"
      title="Play"
      on:click={play}
      disabled={playing}
    >
      <i class="fa-solid fa-play" />
    </button>
  {/if}
  <button
    class="btn w-8 h-6 variant-filled-primary {fastforwarding ? 'variant-filled-secondary' : ''}"
    title="Fast"
    on:click={fastforward}
    disabled={!playing}
  >
    <i class="fa-solid fa-forward" />
  </button>
  <button
    class="btn w-8 h-6 variant-filled-primary {$canvasSettings.tracking ? 'variant-filled-secondary' : ''}"
    title="Fast"
    on:click={trackSelection}
  >
    <i class="fa-solid fa-magnifying-glass" />
  </button>
  <span class="w-12 overflow-hidden">{average}</span>
</div>