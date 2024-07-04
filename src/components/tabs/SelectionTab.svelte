<script lang="ts">
  import RangeInput from "$components/RangeInput.svelte";
    import type { XY } from "$lib/point";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let index = 0;
  let controls = {
    sail_angle: 0.0,
    rudder_angle: 0.0,
  };
  type Ship = {
    loc: XY,
    vel: XY,
    rot_vel: number,
    heading: number,
    sail_angle: number,
    rudder_angle: number,
  }
  let selection: Ship | undefined;

  onMount(() => {
    getValues();
  });

  async function getValues() {
    let ship = await invoke('get_ship', {index: index}) as Ship;
    controls.sail_angle = ship.sail_angle;
    controls.rudder_angle = ship.rudder_angle;
    selection = ship;
  }

  async function update() {
    let inputs =  {index: index, ...controls};
    await invoke('set_ship_controls', inputs);
    getValues();
  }

</script>

<RangeInput
  name={"Sail Angle"}
  bind:value={controls.sail_angle}
  min={-3.14}
  max={3.14}
  step={0.01}
  {update}
/>
<RangeInput
  name={"Rudder Angle"}
  bind:value={controls.rudder_angle}
  min={-3.14/2}
  max={3.14/2}
  step={0.01}
  {update}
/>

{#if selection != undefined}
  <table class="w-full mt-2 border border-surface-700-200-token">
    <tbody>
      <tr>
        <td class="font-bold">Location</td>
        <td>
          <div class="grid grid-cols-2 text-right">
            <span>{selection.loc.x.toFixed(2)}</span>
            <span>{selection.loc.y.toFixed(2)}</span>
          </div>
        </td>
      </tr>
      <tr>
        <td class="font-bold">Velocity</td>
        <div class="grid grid-cols-2 text-right">
          <span>{selection.vel.x.toFixed(2)}</span>
          <span>{selection.vel.y.toFixed(2)}</span>
        </div>
      </tr>
      <tr>
        <td class="font-bold">Rot. Vel.</td>
        <td class="text-right">{selection.rot_vel.toFixed(2)}</td>
      </tr>
      <tr>
        <td class="font-bold">Heading</td>
        <td class="text-right">{selection.heading.toFixed(2)}</td>
      </tr>
    </tbody>
  </table>
{/if}