<script lang="ts">
    import Tab from "./Tab.svelte";
    import { tabsCache } from "$lib/tab";

    function handleClick(index: number) {
        $tabsCache.selected = index;
    }

    function handleAddTabClick() {
        addDefaultTab();
    }
    
    function addDefaultTab() {
        $tabsCache.selected = $tabsCache.tabs.length;
        $tabsCache.tabs.push({
            name: "Home",
            dir: "/Home/"
        });
    }

    function handleAuxClick(event: MouseEvent, index: number) {
        if (event.button == 1) {
            if ($tabsCache.tabs.length == 1) {
                return;
            }

            // keep selection the same after adjusting array
            if ($tabsCache.selected >= index) {
                $tabsCache.selected -= 1;
            }

            $tabsCache.tabs = [...$tabsCache.tabs.slice(0, index), ...$tabsCache.tabs.slice(index + 1, $tabsCache.tabs.length)];
        }
    }
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />

{#each $tabsCache.tabs as tab, i}
    <Tab open={$tabsCache.selected == i} name={tab.name} on:click={() => handleClick(i)} on:auxclick={(event) => handleAuxClick(event, i)}/>
{/each}

<button class="border-t border-l border-r rounded-t" on:click={() => handleAddTabClick()}>
    <span class="material-symbols-outlined">
    add
    </span>
</button>