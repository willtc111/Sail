<script lang="ts">

function startNew() {
  console.log("Starting new project");
}
function load() {
  console.log("Loading project");
}
function save() {
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