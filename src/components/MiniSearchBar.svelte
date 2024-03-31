<script lang="ts">
	import type { DirectoryInfo } from "$lib/files";

    export let info: DirectoryInfo;
    let input: string;
    
    $: if (input != undefined) {
        console.log(info.folders.filter((name) => name.toLowerCase().includes(input.toLowerCase())));
    }
</script>

<input bind:value={input}/>