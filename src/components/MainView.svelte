<script lang="ts">
    import { tabsCache } from "$lib/tab";
	import FileBrowser from "./FileBrowser.svelte";
	import Home from "./Home.svelte";
</script>

<div class="flex min-h-0 overflow-auto place-content-center size-full">
    {#if $tabsCache.tabs[$tabsCache.selected].dir === "/Home/"}
        <Home/>
    {:else}
        <FileBrowser/>
    {/if}
</div>