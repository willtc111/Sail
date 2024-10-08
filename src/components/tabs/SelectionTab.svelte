<script lang="ts">
  import RangeInput from "$components/RangeInput.svelte";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import PrecisionRangeInput from "$components/PrecisionRangeInput.svelte";
  import { selection } from "$lib/stores/selection";
  import type { ShipData } from "$lib/types";
    import { simulationStep } from "$lib/stores/step";

  let ship_id: number|null = null;
  let controls = {
    sail_inputs: [0.0],
    rudder_input: 0.0,
  };
  let ship: ShipData | null;

  onMount(() => {
    getValues();
  });

  async function getValues() {
    if (ship_id != null) {
      ship = await invoke('get_ship', {index: ship_id}) as ShipData;
      controls.sail_inputs = ship.mainsheet_lengths;
      controls.rudder_input = ship.rudder_angle;
    } else {
      ship = null;
    }
  }

  async function update() {
    let inputs =  {
      index: ship_id,
      mainsheet_lengths: controls.sail_inputs,
      rudder_angle: controls.rudder_input
    };
    await invoke('set_ship_controls', inputs);
  }

  $: subscription($selection, $simulationStep);
  function subscription(selection: number|null, step: number) {
    ship_id = selection;
    getValues();
  }

</script>


{#if ship != undefined}
  <RangeInput
    name={"Sail Angle"}
    bind:value={controls.sail_inputs[0]}
    min={0}
    max={9.8}
    step={0.01}
    reset={0.0}
    {update}
  />
  <PrecisionRangeInput
    name={"Rudder Angle"}
    bind:value={controls.rudder_input}
    min={-3.14/2}
    max={3.14/2}
    step={0.01}
    {update}
  />
  <table class="w-full mt-2 border border-surface-700-200-token">
    <tbody>
      <tr>
        <td class="font-bold">Location</td>
        <td>
          <div class="grid grid-cols-2 text-right">
            <span>{ship.loc.x.toFixed(2)}</span>
            <span>{ship.loc.y.toFixed(2)}</span>
          </div>
        </td>
      </tr>
      <tr>
        <td class="font-bold">Velocity</td>
        <div class="grid grid-cols-2 text-right">
          <span>{ship.vel.x.toFixed(2)}</span>
          <span>{ship.vel.y.toFixed(2)}</span>
        </div>
      </tr>
      <tr>
        <td class="font-bold">Rot. Vel.</td>
        <td class="text-right">{ship.rot_vel.toFixed(2)}</td>
      </tr>
      <tr>
        <td class="font-bold">Heading</td>
        <td class="text-right">{ship.heading.toFixed(2)}</td>
      </tr>
    </tbody>
  </table>
{:else}
  <p>
    Select a ship.
  </p>
{/if}