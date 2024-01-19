<script lang="ts">
	import type { TabInfo } from "$lib/tab";

    export let info: TabInfo;

    export let color = "bg-red-700";
</script>

<button class="hover:bg-[#e6497d] {color} text-white font-bold py-2 px-4 rounded-tl-lg rounded-tr-lg min-w-40 border-l-2 border-t-2 border-r-2 border-pink-800" on:click on:auxclick>
    {info.name}
</button>