<script lang="ts">
	import Tab from "./Tab.svelte";
    import { createEventDispatcher } from "svelte";

    export let ids: number[];
    export let selectedId: number;

    const dispatch = createEventDispatcher();

    function addTab() {
        dispatch('addTab');
    }

    function selectTab(id: number) {
        dispatch('selectTab', {
            id,
        });
    }

    function removeTab(id: number) {
        dispatch('removeTab', {
            id,
        });
    }

    function handleAuxClick(event: MouseEvent, id: number) {
        if (event.button == 1) {
            removeTab(id);
        }
    }
    
    // function setContextMenu(index: number) {
    //     $contextMenuInfo.buttons = 
    //     [
    //         {
    //             title: "Close Tab",
    //             callback: () => { closeTab(index) }
    //         }
    //     ];

    //     $contextMenuInfo.isShowing = true;
    // }
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />

{#each ids as id (id)}
    <Tab open={id == selectedId} name={id.toString()} on:click={() => selectTab(id)} on:auxclick={(event) => handleAuxClick(event, id)}/>
{/each}

<button class="border-t border-l border-r rounded-t" on:click={addTab}>
    <span class="material-symbols-outlined">
    add
    </span>
</button>