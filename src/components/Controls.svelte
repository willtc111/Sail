<script lang="ts">

  export let draw: () => void;
  export let home: () => void;

  let loopTimeoutId: NodeJS.Timeout | undefined;
  function pause() {
    stopLoop();
    playing = false;
    fastforwarding = false;
  }

  function step() {
    console.log("step!");
    draw();
  }

  function loopStep() {
    step();
    loopTimeoutId = setTimeout(
      () => loopStep(),
      fastforwarding ? 1 : 100
    );
  }

  function stopLoop() {
    if (loopTimeoutId != undefined) {
      clearTimeout(loopTimeoutId);
      loopTimeoutId = undefined;
    }
  }

  let playing = false;
  function play() {
    stopLoop();
    playing = true;
    loopStep();
  }

  let fastforwarding = false;
  function fastforward() {
    fastforwarding = !fastforwarding;
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
</div>