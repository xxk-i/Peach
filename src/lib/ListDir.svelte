<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  // main.rs returns to us a Vec<String>
  let dirs = [];

  async function listdir() {
    // this is the rust call
    dirs = await invoke("listdir")
  }
</script>

<div>
  <div class="row">
    <button on:click={listdir}>
      List Directories
    </button>
  </div>
  <!-- collection as item -->
  {#each dirs as dir}
  <h2>{dir}</h2>
  {/each}
</div>