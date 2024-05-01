<script lang="ts">
    export let filter: string;

    // $: if (input != undefined) {
    //     console.log(info.folders.filter((name) => name.toLowerCase().includes(input.toLowerCase())));
    // }
</script>

<input bind:value={filter}/>