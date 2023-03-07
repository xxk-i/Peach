<script lang="ts">
  import ListDir from './lib/ListDir.svelte'

  let path = "";
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>
  <input bind:value={path}>

  <h1>Hello!</h1>

  <div class="row">
    <ListDir path={path}/>
  </div>

</main>