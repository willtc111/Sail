<script lang="ts">
  import { controlsInterface } from "$lib/stores/controls";
  import { invoke } from "@tauri-apps/api";


  async function startNew() {
    console.log("Starting new project");
    $controlsInterface.pause();
    await invoke('reset_simulation');
    $controlsInterface.redraw();
  }
  async function load() {
    console.log("Loading project");
    $controlsInterface.pause();
  }
  async function save() {
    console.log("Saving project");
    $controlsInterface.pause();
  }

</script>

<div>
  <button
    class="btn variant-filled-primary"
    on:click={startNew}
  >
    New
  </button>
  <button
    class="btn variant-filled-primary"
    on:click={load}
  >
    Load
  </button>
  <button
    class="btn variant-filled-primary"
    on:click={save}
  >
    Save
  </button>
</div>