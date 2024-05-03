<script lang="ts">
	import Tab from "./Tab.svelte";
    import { contextMenuInfo } from "$lib/global";
    import { tabStore } from "$lib/stores";
	import { get } from "svelte/store";

    function addTab() {
        tabStore.addTab();
    }

    function addTabAndSwitch() {
        tabStore.addTabAndSelectIt();
    }

    function selectTab(id: number) {
        tabStore.setSelected(id);
    }

    function removeTab(id: number) {
        tabStore.closeTab(id);
    }

    function handleAuxClick(event: MouseEvent, id: number) {
        if (event.button == 1) {
            removeTab(id);
        }
    }
    
    function setContextMenu(id: number) {
        $contextMenuInfo.buttons = 
        [
            {
                title: "Close Tab",
                callback: () => { removeTab(id) }
            }
        ];

        $contextMenuInfo.isShowing = true;
    }
    
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />

{#each $tabStore.infos as info (get(info).id)}
    <Tab {info}
        on:click={() => selectTab(get(info).id)}
        on:auxclick={(event) => handleAuxClick(event, get(info).id)}
        on:contextmenu={() => setContextMenu(get(info).id)}/>
{/each}

<!-- TODO proper sticky add tab button -->
<button class="border-t border-l border-r rounded-t bg-peach-500 border-peach-400" on:click={addTabAndSwitch}>
    <span class="material-symbols-outlined">
    add
    </span>
</button>