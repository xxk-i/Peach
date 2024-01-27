<script lang="ts">
    import Tab from "./Tab.svelte";
    import { createEventDispatcher, onMount } from "svelte";
    import { tabsInfo } from "$lib/tab";

    const dispatch = createEventDispatcher();

    onMount(() => {
        dispatchAddTabEvent(-1);
    });

    function handleClick(index: number) {
        if (index == $tabsInfo.selected) {
            return
        }

        let oldIndex = $tabsInfo.selected;
        $tabsInfo.selected = index;
        dispatchSwitchTabEvent(oldIndex);
    }

    function addTab() {
        let previousIndex = $tabsInfo.selected;
        $tabsInfo.selected = $tabsInfo.count;
        $tabsInfo.count += 1;
        dispatchAddTabEvent(previousIndex);
    }

    function dispatchAddTabEvent(previousIndex: number) {
        dispatch('addTab', {
            previousIndex,
            dir: "/Home/"
        });
    }

    function dispatchCloseTabEvent(index: number) {
        dispatch('closeTab', {
            index: index,
        });
    }

    function dispatchSwitchTabEvent(oldIndex: number) {
        dispatch('switchTab', {
            oldIndex,
            newIndex: $tabsInfo.selected,
        });
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
            dispatchCloseTabEvent(index);
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