<script lang="ts">
	import { tabsInfo } from "$lib/tab";
	import { setContext } from "svelte";
	import FileBrowser from "./FileBrowser.svelte";
	import Home from "./Home.svelte";
	import { writable } from "svelte/store";

    export let dir = writable("");
    export let index: number;

    setContext("dir", dir);
</script>

<div class="grow overflow-auto {index == $tabsInfo.selected ? "" : "hidden"}">
    {#if $dir === "/Home/"}
        <Home/>
    {:else}
        <FileBrowser/>
    {/if}
</div>