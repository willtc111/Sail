<script lang="ts">
  import Canvas from "$components/Canvas.svelte";
  import Controls from "$components/Controls.svelte";
  import ForcesGraph from "$components/tabs/physics/ForcesGraph.svelte";
  import PhysicsDebug from "$components/tabs/physics/PhysicsDebug.svelte";
  import ProjectTab from "$components/tabs/ProjectTab.svelte";
  import SelectionTab from "$components/tabs/SelectionTab.svelte";
  import SettingsTab from "$components/tabs/SettingsTab.svelte";
  import { XY } from "$lib/point";
  import { canvasClick } from "$lib/stores/canvasInterface";
  import { selection } from "$lib/stores/selection";
  import { LightSwitch, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api";
  import { onDestroy } from "svelte";

  // Window dimensions (for responsive resizing)
  let innerWidth: number;
  let innerHeight: number;
  $: cardHeight = innerHeight - 136;

  let tabSet: number = 3;

  const unsubscribe = canvasClick.subscribe((value: XY) => selectShip(value));
  onDestroy(unsubscribe);
  async function selectShip(loc: XY) {
    let ship_id = await invoke('get_ship_id', {loc: loc}) as number|null;
    console.log(`Click at ${loc} selected ship: ${ship_id}`);
    $selection = ship_id;
    if (ship_id != null && tabSet != 3) {
      tabSet = 2;
    }
  }
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<div class="h-full flex p-1.5 gap-1.5">
  <!-- Display -->
  <div class="grow shrink">
    <Canvas/>
  </div>

  <!-- Info/Controls -->
  <div class="h-full w-96 flex flex-col gap-2">
    <section class="card flex justify-between p-1">
      <h1 class="font-mono font-bold tracking-widest">Sail</h1>
      <LightSwitch />
    </section>
    <section class="card">
      <Controls dishWidth={10000} dishHeight={10000}></Controls>
    </section>
    <section class="card grow">
      <TabGroup regionPanel={"!mt-0 !m-0 !p-0"}>
        <Tab bind:group={tabSet} name="tab1" value={0}>Project</Tab>
        <Tab bind:group={tabSet} name="tab2" value={1}>Settings</Tab>
        <Tab bind:group={tabSet} name="tab3" value={2}>Selection</Tab>
        <Tab bind:group={tabSet} name="tab4" value={3}>Physics</Tab>
        <!-- Tab Panels --->
        <div slot="panel" class="p-2 overflow-y-auto" style="height: {cardHeight}px;">
          {#if tabSet === 0}
            <ProjectTab/>
          {:else if tabSet === 1}
            <SettingsTab/>
          {:else if tabSet === 2}
            <SelectionTab />
          {:else if tabSet === 3}
            <PhysicsDebug />
            <ForcesGraph/>
          {/if}
        </div>
      </TabGroup>
    </section>
  </div>
</div>

<style>
</style>


