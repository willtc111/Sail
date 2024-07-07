<script lang="ts">
  export let name: string;
  export let color: string = '#000000';
  export let value: number;
  export let min: number;
  export let max: number;
  export let step: number;
  export let reset: number = (max - min) / 2.0 + min;
  export let update: () => {};

  let slideMin = Math.sign(min) * Math.sqrt(Math.abs(min));
  let slideMax = Math.sign(max) * Math.sqrt(Math.abs(max));
  let slideVal = Math.sign(value) * Math.sqrt(Math.abs(value));
  let slideStep = step/2;

  $: if (value) {
    updateRaw();
  }

  function updateRaw() {
    slideVal = Math.sign(value) * Math.sqrt(Math.abs(value));
    update();
  }

  function updateSlide() {
    value = Math.sign(slideVal) * slideVal * slideVal;
    update();
  }

  function resetValue() {
    value = reset;
    updateRaw();
  }

</script>

<div class="flex flex-row flex-wrap justify-between">
  <label for={name} class="basis-1/3 font-bold" style="color: {color};">
    {name}
  </label>
  <button
    on:click={resetValue}
    class="basis-1/6 px-2 text-right"
  >
    <i class="fa-solid fa-refresh"/>
  </button>
  <input
    {name}
    {min}
    {max}
    {step}
    bind:value
    on:input={updateRaw}
    type="number"
    class="basis-3/6 input text-right border-l-2 border-l-black"
  >
  <input
    {name}
    min={slideMin}
    max={slideMax}
    step={slideStep}
    bind:value={slideVal}
    on:input={updateSlide}
    type="range"
    class="basis-full py-0.5"
    style="accent-color: {color};"
  />
</div>