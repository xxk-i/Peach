<script lang="ts">
	import { SpecialPath, type TabInfo } from "$lib/stores";
	import { getContext } from "svelte";
	import type { Writable } from "svelte/store";
	import FileBrowser from "./FileBrowser.svelte";

    let tabInfo: Writable<TabInfo> = getContext("tabInfo");
</script>

<FileBrowser/>