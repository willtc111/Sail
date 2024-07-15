<script lang="ts">
  import RangeInput from "$components/RangeInput.svelte";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let parameters = {
    wind_angle: 0.0,
    wind_speed: 0.0,
  };

  onMount(() => {
    reset();
  });

  async function reset() {
    parameters = await invoke('get_sim_settings');
  }

  async function update() {
    await invoke('set_sim_settings', parameters);
  }
</script>


<RangeInput
  name={"Wind Angle"}
  bind:value={parameters.wind_angle}
  min={-3.14}
  max={3.14}
  step={0.01}
  {update}
/>
<RangeInput
  name={"Wind Speed"}
  bind:value={parameters.wind_speed}
  min={0}
  max={5.0}
  step={0.1}
  reset={0.0}
  {update}
/>