<script lang="ts">
    import { mousePosition, type ContextMenuButton } from "$lib/global";
    import { contextMenuInfo } from "$lib/global";
	import { onDestroy } from "svelte";

    let left = 0;
    let top = 0;
    let isShowing = false;
    let buttons: ContextMenuButton[] = [];

    let unsubscribe = contextMenuInfo.subscribe((value) => {
        if ($mousePosition) {
            left = $mousePosition.x;
            top = $mousePosition.y;
        }
        isShowing = value.isShowing;
        buttons = value.buttons;
    });

    function onLeftClick() {
        isShowing = false;
    }

    onDestroy((unsubscribe));
</script>

{#if isShowing}
    <div class="absolute flex flex-col rounded-lg border-[solid] border-peach-800;" style="left: {left + "px"}; top: {top + "px"};">
        {#each buttons as button}
            <button class="text-left" on:click={() => button.callback()}>{button.title}</button>
        {/each}
    </div>
{/if}

<style>
    .context_menu {
        background: #e6497d;
        border: solid;
        border-color: rgb(124, 8, 47);
        position: absolute;
        border-radius: 0.5rem;
        /* width: 10rem; */
        display: flex;
        flex-direction: column;
    }
</style>

<!-- Disable right click and enable left click capture -->
<svelte:window on:contextmenu|preventDefault on:click={onLeftClick}/>