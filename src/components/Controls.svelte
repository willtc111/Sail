<script lang="ts">
  import { Axis, Rectangle, Ship } from "$lib/drawing";
  import { RollingAverage } from "$lib/performance";
  import { XY } from "$lib/point";
  import { canvasInterface, canvasSettings, drawBuffer } from "$lib/stores/canvasInterface";
    import { controlsInterface } from "$lib/stores/controls";
  import { selection } from "$lib/stores/selection";
    import { simulationStep } from "$lib/stores/step";
  import { invoke } from "@tauri-apps/api";

  export let dishWidth: number;
  export let dishHeight: number;

  export let stepDelayMs: number = 1000/60; // 60 fps

  let {draw, centerOn} = $canvasInterface;
  controlsInterface.set({
    home: home,
    step: stepUpdate,
    play: play,
    pause: pause,
    fastforward: fastforward,
    toggleTracking: toggleTracking,
    redraw: stepDraw,
  });

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
    $simulationStep = stepCount;
    await stepDraw();
    let curTime = Date.now();
    let elapsed = curTime - lastTime;
    lastTime = curTime;
    mspf.add(elapsed);
    average = mspf.get() ?? "---";
  }

  stepDraw();

  async function stepDraw() {
    let halfWidth = dishWidth / 2;
    let halfHeight = dishHeight / 2;
    drawBuffer.set([
      new Axis(
        new XY(halfWidth, halfHeight),
        new XY(50, 50),
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
      s.hull,
      s.sails,
      s.rudder,
      'brown', 'white'));
    ships.forEach(d => drawBuffer.add(d));

    trackSelection();
    draw();
  }

  function loopStep() {
    stepUpdate();
    loopTimeoutId = setTimeout(
      () => loopStep(),
      fastforwarding ? 0 : stepDelayMs
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

  function toggleTracking() {
    if ($selection != null) {
      $canvasSettings.tracking = !$canvasSettings.tracking;
      if ($canvasSettings.tracking) {
        trackSelection();
      }
    } else {
      $canvasSettings.tracking = false;
    }
  }

  $: $canvasSettings.tracking = $canvasSettings.tracking && $selection != null;

  async function trackSelection() {
    if ($selection != null) {
      let ship = await invoke('get_ship', {index: $selection}) as {loc: XY};
      if (ship != null && $canvasSettings.tracking) {
        centerOn(XY.from(ship.loc), undefined);
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
    disabled={$selection == null}
    title="Fast"
    on:click={toggleTracking}
  >
    <i class="fa-solid fa-magnifying-glass" />
  </button>
  <span class="w-12 overflow-hidden">{average}</span>
</div>