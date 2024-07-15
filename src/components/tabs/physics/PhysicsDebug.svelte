<script lang="ts">
  import { Arrow, Ship } from "$lib/drawing";
  import { XY } from "$lib/point";
  import { invoke } from '@tauri-apps/api/tauri';
  import RangeInput from "../../RangeInput.svelte";
  import { selection } from "$lib/stores/selection";
  import type { ShipData } from "$lib/types";
  import { simulationStep } from "$lib/stores/step";

  // Canvas dimensions
  let width = 300;
  let height = 300;

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;
  $: if (canvas) {
    ctx = canvas.getContext('2d', { alpha: false });
    if ($simulationStep && syncSelection && $selection != null) {
      loadFromSelection();
    } else {
      update();
    }
  }

  // let sail_max = 3.14; // square rig
  let sail_max = Math.sqrt(7*7+7*7); // fore-aft rig

  let temp_parameters = {
    move_angle: 0.0,
    move_speed: 0.0,
  }
  let parameters = {
    wind_angle: 0.0,
    wind_speed: 5.0,
    velocity: new XY(0.0, 0.0),
    rot_velocity: 0.0,
    heading: 0.0,
    sail: 0.0,
    rudder_angle: 0.0
  };
  let colors = {
    velocity: '#ff0000',
    wind: '#0000ff',
    apparent_wind: '#8888ff',
    sail_lift: '#00ff00',
    sail_drag: '#ff0000',
    keel_lift: '#00ffff',
    keel_drag: '#ff00ff',
    hull_drag: '#ff8888',
    rudder_lift: '#00ffff',
    rudder_drag: '#ff00ff',
  };
  let forceToggles = {
    sail: true,
    keel: true,
    hull: true,
    rudder: true,
  };

  let syncSelection = $selection != null;
  $: syncSelection = syncSelection && $selection != null;

  $: if ($simulationStep && syncSelection && $selection != null) {
    loadFromSelection();
  }

  async function loadFromSelection() {
    let ship = await invoke('get_ship', {index: $selection}) as ShipData;
    parameters.velocity = XY.from(ship.vel);
    temp_parameters.move_angle = parameters.velocity.direction();
    temp_parameters.move_speed = parameters.velocity.magnitude();
    parameters.rot_velocity = ship.rot_vel;
    parameters.heading = ship.heading;
    parameters.sail = ship.mainsheet_length;
    parameters.rudder_angle = ship.rudder_angle;
    let settings = await invoke('get_sim_settings') as { wind_angle: number, wind_speed: number };
    parameters.wind_angle = settings.wind_angle;
    parameters.wind_speed = settings.wind_speed;
    update();
  }

  async function update() {
    if (ctx == null) { return; }

    parameters.velocity.x = Math.cos(temp_parameters.move_angle) * temp_parameters.move_speed;
    parameters.velocity.y = Math.sin(temp_parameters.move_angle) * temp_parameters.move_speed;

    let shapes = await invoke('debug_ship_physics', parameters) as {
      ship: any,
      forces: any[]
    };
    let ship = new Ship(
      shapes.ship.center,
      shapes.ship.hull,
      shapes.ship.sail,
      shapes.ship.rudder,
      'brown',
      'white'
    );
    let forces: Arrow[] = [];

    function forceToArrow(f: any, color: string, bonusWidth: number = 0.0): Arrow {
      return new Arrow(f.start, f.end, f.width + bonusWidth, f.head_size, color);
    }

    forces.push(forceToArrow(shapes.forces[1], colors.velocity, 0.5)); // velocity
    forces.push(forceToArrow(shapes.forces[0], colors.wind)); // wind
    forces.push(forceToArrow(shapes.forces[2], colors.apparent_wind)); // apparent wind
    forces.push(forceToArrow(shapes.forces[3], colors.velocity)); // rotation
    if (parameters.wind_speed != 0 || parameters.velocity.magnitude() != 0) {
      if (forceToggles.sail) {
        forces.push(forceToArrow(shapes.forces[4], colors.sail_lift)); // sail lift
        forces.push(forceToArrow(shapes.forces[5], colors.sail_drag)); // sail drag
      }
    }
    if (parameters.velocity.magnitude() != 0 || parameters.rot_velocity != 0) {
      if (forceToggles.keel) {
        forces.push(forceToArrow(shapes.forces[6], colors.keel_lift, 0.5)); // keel lift
        forces.push(forceToArrow(shapes.forces[7], colors.keel_drag, 0.5)); // keel drag
        forces.push(forceToArrow(shapes.forces[8], colors.keel_lift, 0.5)); // keel lift
        forces.push(forceToArrow(shapes.forces[9], colors.keel_drag, 0.5)); // keel drag
      }
      if (forceToggles.rudder) {
        forces.push(forceToArrow(shapes.forces[10], colors.rudder_lift, 0.125)); // rudder lift
        forces.push(forceToArrow(shapes.forces[11], colors.rudder_drag, 0.125)); // rudder drag
      }
      if (forceToggles.hull) {
        forces.push(forceToArrow(shapes.forces[12], colors.hull_drag, 0.5)); // hull front drag
        forces.push(forceToArrow(shapes.forces[13], colors.hull_drag, 0.5)); // hull rear drag
      }
    }

    ctx.reset();
    // Flip y axis to be "normal"
    ctx.setTransform(10,0,0,-10,canvas.width/2,canvas.height/2);

    ship.draw(ctx);
    forces.forEach(f => f.draw(ctx!));
  }
</script>

<div>
  <canvas
    bind:this={canvas}
    {width}
    {height}
    class="w-full h-full rounded-container-token"
  />
  <div class="flex justify-between">
    <div class="flex gap-1">
      <label>
        <span class="font-bold">S</span>
        <span class="font-bold" style="color: {colors.sail_lift};">L</span>
        <span class="font-bold" style="color: {colors.sail_drag};">D</span>
        <input type="checkbox" bind:checked={forceToggles.sail} on:change={update}>
      </label>
      <label>
        <span class="font-bold">K</span>
        <span class="font-bold" style="color: {colors.keel_lift};">L</span>
        <span class="font-bold" style="color: {colors.keel_drag};">D</span>
        <input type="checkbox" bind:checked={forceToggles.keel} on:change={update}>
      </label>
      <label>
        <span class="font-bold">H</span>
        <span class="font-bold" style="color: {colors.hull_drag};">D</span>
        <input type="checkbox" bind:checked={forceToggles.hull} on:change={update}>
      </label>
      <label>
        <span class="font-bold">R</span>
        <span class="font-bold" style="color: {colors.rudder_lift};">L</span>
        <span class="font-bold" style="color: {colors.rudder_drag};">D</span>
        <input type="checkbox" bind:checked={forceToggles.rudder} on:change={update}>
      </label>
    </div>
    <div class="">
      <label class="flex flex-row gap-1">
        <span class="font-bold">Sync</span>
        <div class="w-4">
          {#if $selection == null}
            <i class="fa fa-info-circle" title="Select a ship to toggle syncing"/>
          {/if}
          <input type="checkbox" bind:checked={syncSelection} hidden={$selection == null}/>
        </div>
      </label>
    </div>
  </div>
  <div class="py-1">
    <RangeInput
      name={"Wind Angle"}
      color={colors.wind}
      bind:value={parameters.wind_angle}
      min={-3.14}
      max={3.14}
      step={0.01}
      disabled={syncSelection}
      {update}
    />
    <RangeInput
      name={"Wind Speed"}
      color={colors.wind}
      bind:value={parameters.wind_speed}
      min={0}
      max={10.0}
      step={0.01}
      reset={1.0}
      disabled={syncSelection}
      {update}
    />
    <RangeInput
      name={"Sail Input"}
      bind:value={parameters.sail}
      min={0.0}
      max={sail_max}
      step={0.01}
      disabled={syncSelection}
      {update}
    />
    <RangeInput
      name={"Rudder Angle"}
      bind:value={parameters.rudder_angle}
      min={-3.14/2}
      max={3.14/2}
      step={0.01}
      disabled={syncSelection}
      {update}
    />
    <RangeInput
      name={"Heading"}
      bind:value={parameters.heading}
      min={-3.14}
      max={3.14}
      step={0.01}
      disabled={syncSelection}
      {update}
    />
    <RangeInput
      name={"Vel Angle"}
      color={colors.velocity}
      bind:value={temp_parameters.move_angle}
      min={-3.14}
      max={3.14}
      step={0.01}
      reset={0.0}
      disabled={syncSelection}
      {update}
    />
    <RangeInput
      name={"Vel Speed"}
      color={colors.velocity}
      bind:value={temp_parameters.move_speed}
      min={0}
      max={10}
      step={0.01}
      reset={0.0}
      disabled={syncSelection}
      {update}
    />
    <RangeInput
      name={"Rot Vel"}
      color={colors.velocity}
      bind:value={parameters.rot_velocity}
      min={-5}
      max={5}
      step={0.01}
      reset={0.0}
      disabled={syncSelection}
      {update}
    />
  </div>
</div>