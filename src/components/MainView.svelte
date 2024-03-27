<script lang="ts">
	import FileBrowser from "./FileBrowser.svelte";
	import Home from "./Home.svelte";
    import Applications from "./Applications.svelte";
	import { setContext } from "svelte";
	import { type Writable } from "svelte/store";
	import { tabStore } from "$lib/stores";
    import { type TabInfo } from "$lib/stores";

    export let tabInfo: Writable<TabInfo>;

    setContext("tabInfo", tabInfo);
</script>

<div class="mainview basis-full {$tabInfo.id != $tabStore.selected ? "hidden" : ""}">
    {#if $tabInfo.directory === "/Home/"}
        <Home/>
    {:else if $tabInfo.directory === "/Applications/"}
        <Applications/>
    {:else}
        <FileBrowser/>
    {/if}
</div>