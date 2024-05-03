<script lang="ts">
	import { tabStore, type TabInfo } from "$lib/stores";
	import type { Writable } from "svelte/store";

    export let info: Writable<TabInfo>;
</script>
<button
class="hover:bg-peach-500 {$info.id == $tabStore.selected ? "bg-peach-500" : "bg-peach-600"} text-white font-bold py-0.5 px-4 rounded-tl-lg rounded-tr-lg min-w-40 border-l-2 border-t-2 border-r-2 border-peach-400"
on:click
on:auxclick
on:contextmenu>
    {$info.name}
</button>