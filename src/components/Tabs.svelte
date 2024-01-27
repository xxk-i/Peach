<script lang="ts">
    import Tab from "./Tab.svelte";
    import { tabsInfo } from "$lib/tab";

    function handleClick(index: number) {
        $tabsInfo.selected = index;
    }
    
    function addTab() {
        $tabsInfo.count += 1;
    }

    function handleAuxClick(event: MouseEvent, index: number) {
        if (event.button == 1) {
            if ($tabsInfo.count == 1) {
                return;
            }

            // keep selection the same after adjusting array
            if ($tabsInfo.selected >= index) {
                $tabsInfo.selected -= 1;
            }

            $tabsInfo.count -= 1;
        }
    }
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />

{#each Array($tabsInfo.count) as _, i}
    <Tab open={$tabsInfo.selected == i} name={"Hello"} on:click={() => handleClick(i)} on:auxclick={(event) => handleAuxClick(event, i)}/>
{/each}

<button class="border-t border-l border-r rounded-t" on:click={addTab}>
    <span class="material-symbols-outlined">
    add
    </span>
</button>