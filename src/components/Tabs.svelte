<script lang="ts">
    import Tab from "./Tab.svelte";

    let selectedTab = 0;
    let tabCount = 4;

    function handleClick(index: number) {
        console.log("selectedTab: " + selectedTab);
        console.log("index: " + index);
        if (index == selectedTab) {
            return;
        } else {
            selectedTab = index;
        }
    }
</script>

{#each Array(tabCount) as _, i}
    {#if selectedTab == i}
        <Tab index={i} color={"bg-[#e6497d]"} on:click={() => handleClick(i)}/>
    {:else}
        <Tab index={i} on:click={() => handleClick(i)}/>
    {/if}
{/each}