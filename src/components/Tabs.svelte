<script lang="ts">
    import type { TabInfo } from "$lib/tab.ts";
    import Tab from "./Tab.svelte";
    import { onMount } from "svelte";

    let selectedTab = 0;
    let tabs: TabInfo[] = [];

    onMount(() => {
        addDefaultTab();
    });

    function handleClick(index: number) {
        console.log("selectedTab: " + selectedTab);
        console.log("index: " + index);
        if (index == selectedTab) {
            return;
        } else {
            selectedTab = index;
        }
    }

    function handleAddTabClick() {
        addDefaultTab();
    }
    
    function addDefaultTab() {
        tabs = [...tabs, {
            index: tabs.length,
            name: "Home",
            dir: ""
        }];

        // set selection to newest tab
        selectedTab = tabs.length - 1;
    }

    function handleTabMiddleClick(index: number) {
        if (tabs.length == 1) {
            return;
        }

        // keep selection the same after adjusting array
        if (selectedTab >= index) {
            selectedTab -= 1;
        }

        tabs = [...tabs.slice(0, index), ...tabs.slice(index, tabs.length - 1)];
    }
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />

{#each tabs as tab}
    {#if selectedTab == tab.index}
        <Tab info={tab} color={"bg-[#e6497d]"} on:click={() => handleClick(tab.index)} on:auxclick={() => handleTabMiddleClick(tab.index)}/>
    {:else}
        <Tab info={tab} on:click={() => handleClick(tab.index)} on:auxclick={() => handleTabMiddleClick(tab.index)}/>
    {/if}
{/each}
<button class="border-t border-l border-r rounded-t" on:click={handleAddTabClick}>
    <span class="material-symbols-outlined">
    add
    </span>
</button>