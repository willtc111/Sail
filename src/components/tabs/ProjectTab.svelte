<script lang="ts">
  import { invoke } from "@tauri-apps/api";


  async function startNew() {
    console.log("Starting new project");
    await invoke('reset_simulation');
  }
  async function load() {
    console.log("Loading project");
  }
  async function save() {
    console.log("Saving project");
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