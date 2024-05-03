<script lang="ts">
	import FileBrowser from "./FileBrowser.svelte";
	import Home from "./Home.svelte";
    import Applications from "./Applications.svelte";
    import Sync from "./Sync.svelte";
	import { setContext } from "svelte";
	import { type Writable } from "svelte/store";
	import { tabStore } from "$lib/stores";
    import { type TabInfo, SpecialDirectory } from "$lib/stores";

    export let tabInfo: Writable<TabInfo>;

    setContext("tabInfo", tabInfo);
</script>

<!-- TODO fix this weird height: 0; min-height: 100%; meme (prevents growing in cross-axis direction)-->
<!-- EDIT: probably the best way <3 css -->
<div class="flex mainview overflow-auto grow basis-full h-0 min-h-full {$tabInfo.id != $tabStore.selected ? "hidden" : ""}">
    {#if $tabInfo.directory === SpecialDirectory.Home}
        <Home/>
    {:else if $tabInfo.directory === SpecialDirectory.Applications}
        <Applications/>
    {:else if $tabInfo.directory === SpecialDirectory.Sync}
        <Sync/>
    {:else}
        <FileBrowser/>
    {/if}
</div>