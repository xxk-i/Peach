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

<div class="mainview {$tabInfo.id != $tabStore.selected ? "hidden" : ""}" style="width: 100%; height: 100%;">
    {#if $tabInfo.directory === "/Home/"}
        <Home/>
    {:else if $tabInfo.directory === "/Applications/"}
        <Applications/>
    {:else}
        <FileBrowser/>
    {/if}
</div>