<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  export let path = "";

  async function listdir() {
    // this is the rust call
    return await invoke("listdir", {dir: path});
  }

  // main.rs returns to us a Vec<String>
  //let dirs = [];

  let promise = listdir();

  function handleListDir() {
    promise = listdir();
  }
</script>

<div>
  <div class="row">
    <button on:click={handleListDir}>
      List Directories
    </button>
  </div>

  {#await promise}
    <h2>...waiting</h2>
  {:then dirs}
  <!-- collection as item -->
   {#each dirs as dir}
   <h2>{dir}</h2>
   {/each}
  {/await}
</div>