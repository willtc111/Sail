<script lang="ts">
  import Canvas from "$components/Canvas.svelte";
  import Controls from "$components/Controls.svelte";
  import ProjectTab from "$components/tabs/ProjectTab.svelte";
  import { LightSwitch, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { invoke } from '@tauri-apps/api/tauri';

  let tabSet: number = 0;
</script>

<div class="flex h-full p-1.5 gap-1.5">
  <!-- Display -->
  <div class="grow shrink ">
    <Canvas/>
    <div class="w-16 bg-red-500"></div>
  </div>

  <!-- Info/Controls -->
  <div class="h-full w-96 flex flex-col gap-2">
    <section class="card flex justify-between p-1">
      <h1 class="font-mono font-bold tracking-widest">Petri</h1>
      <LightSwitch />
    </section>
    <section class="card">
      <Controls dishWidth={1000} dishHeight={1000}></Controls>
    </section>
    <section class="card grow">
      <TabGroup spacing={"space-y-6"}>
        <Tab bind:group={tabSet} name="tab1" value={0}>Project</Tab>
        <Tab bind:group={tabSet} name="tab2" value={1}>Dish</Tab>
        <Tab bind:group={tabSet} name="tab3" value={2}>Selection</Tab>
        <Tab bind:group={tabSet} name="tab4" value={3}>Overview</Tab>
        <!-- Tab Panels --->
        <div slot="panel" class="-mt-2 mx-2 mb-2">
          {#if tabSet === 0}
            <ProjectTab></ProjectTab>
          {:else if tabSet === 1}
            (tab panel 2 contents)
          {:else if tabSet === 2}
            (tab panel 3 contents)
          {:else if tabSet === 3}
            (tab panel 4 contents)
          {/if}
        </div>
      </TabGroup>
    </section>
  </div>
</div>

<style>
</style>


