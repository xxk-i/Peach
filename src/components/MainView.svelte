<script lang="ts">
	import { tabsInfo } from "$lib/tab";
	import { setContext } from "svelte";
	import FileBrowser from "./FileBrowser.svelte";
	import Home from "./Home.svelte";
	import { writable } from "svelte/store";

    export let dir = writable("");

    setContext("dir", dir);
</script>

<div class="mainview" style="width: 100%; height: 100%;">
    {#if $dir === "/Home/"}
        <Home/>
    {:else}
        <FileBrowser/>
    {/if}
</div>