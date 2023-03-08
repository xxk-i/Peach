<script lang="ts">
  import ListDir from './lib/ListDir.svelte'

  let path = "/Users/grojdg/Developer";
</script>

<main class="container">
  <h1>Peach</h1>
  <input bind:value={path}>

  <div class="row">
    <ListDir path={path}/>
  </div>

</main>