<script lang="ts">
	import { setContext } from "svelte";
	import FileBrowser from "./FileBrowser.svelte";
	import Home from "./Home.svelte";
	import { writable } from "svelte/store";

    export let dir = writable("");

    setContext("dir", dir);
</script>

{#if $dir === "/Home/"}
    <Home/>
{:else}
    <FileBrowser/>
{/if}