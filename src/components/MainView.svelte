<script lang="ts">
	import { setContext } from "svelte";
	import FileBrowser from "./FileBrowser.svelte";
	import Home from "./Home.svelte";
    import Applications from "./Applications.svelte";
	import { writable } from "svelte/store";

    export let dir = writable("");
    export let hidden = false;

    setContext("dir", dir);
</script>

<div class="mainview {hidden ? "hidden" : ""}" style="width: 100%; height: 100%;">
    {#if $dir === "/Home/"}
        <Home/>
    {:else if $dir === "/Applications/"}
        <Applications/>
    {:else}
        <FileBrowser/>
    {/if}
</div>