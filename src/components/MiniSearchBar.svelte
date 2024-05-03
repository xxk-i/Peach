<script lang="ts">
	import type { Writable } from "svelte/store";

    export let filter: Writable<string>;

    // $: if (input != undefined) {
    //     console.log(info.folders.filter((name) => name.toLowerCase().includes(input.toLowerCase())));
    // }
</script>

<input bind:value={$filter}/>