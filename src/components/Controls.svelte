<script lang="ts">
    import { Rectangle } from "$lib/drawing";
  import { XY } from "$lib/point";
  import { canvasInterface, canvasSettings, drawBuffer } from "$lib/stores/canvasInterface";

  export let dishWidth: number;
  export let dishHeight: number;

  let {draw, centerOn} = $canvasInterface;

  let loopTimeoutId: NodeJS.Timeout | undefined;
  function pause() {
    stopLoop();
    playing = false;
    fastforwarding = false;
  }

  let test = 0;
  function step() {
    test++;
    let moverLoc = new XY(
      (dishWidth/2 - 10) + 35*Math.cos(test/10),
      (dishHeight/2 - 10) + 35*Math.sin(test/10)
    );
    drawBuffer.set([
      new Rectangle(
        new XY(0, 0),
        new XY(dishWidth, dishHeight),
        undefined, '#0000ff'
      ),
      new Rectangle(
        moverLoc,
        new XY(20, 20),
        'purple'
      ),
      new Rectangle(
        new XY(dishWidth/2, dishHeight/2),
        new XY(5, 5),
        'red'
      ),
      new Rectangle(
        new XY(dishWidth/2, dishHeight/2),
        new XY(-5, -5),
        'blue'
      ),
      new Rectangle(
        new XY(dishWidth/2, dishHeight/2),
        new XY(-1, 1),
        'green'
      ),
      new Rectangle(
        new XY(dishWidth/2, dishHeight/2),
        new XY(1, -1),
        'yellow'
      ),
    ]);
    if ($canvasSettings.tracking) {
      centerOn(moverLoc.add(new XY(10,10)), undefined);
    }
    draw();
  }

  function loopStep() {
    step();
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
    loopStep();
  }

  let fastforwarding = false;
  function fastforward() {
    fastforwarding = !fastforwarding;
  }

  function home() {
    $canvasSettings.tracking = false;
    centerOn(new XY(dishWidth, dishHeight).scale(1/2), 1.0);
  }

  function trackSelection() {
    $canvasSettings.tracking = !$canvasSettings.tracking;
    if ($canvasSettings.tracking) {
      centerOn(
        new XY(
          (dishWidth/2 - 10) + 35*Math.cos(test/10),
          (dishHeight/2 - 10) + 35*Math.sin(test/10)
        ).add(new XY(10,10)
      ), undefined);
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
    title="Pause"
    on:click={pause}
    disabled={!playing}
  >
    <i class="fa fa-pause" />
  </button>
  <button
    class="btn w-8 h-6 variant-filled-primary"
    title="Step"
    on:click={step}
    disabled={playing}
  >
    <i class="fa-solid fa-forward-step" />
  </button>
  <button
    class="btn w-8 h-6 variant-filled-primary"
    title="Play"
    on:click={play}
    disabled={playing}
  >
    <i class="fa-solid fa-play" />
  </button>
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
</div>