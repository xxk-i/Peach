<script lang="ts">
    import { mousePosition } from "$lib/global";
    import { contextMenuInfo } from "$lib/global";
	import { onDestroy } from "svelte";

    let left = 0;
    let top = 0;
    let isShowing = false;
    let buttons;

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
    <div class="context_menu" style="left: {left + "px"}; top: {top + "px"};">
        {#each $contextMenuInfo.buttons as button}
            <button class="text-left" on:click={() => button.callback()}>{button.title}</button>
        {/each}
    </div>
{/if}

<style>
    .context_menu {
        background: rgb(230, 73, 125);
        border: solid;
        border-color: rgb(124, 8, 47);
        position: absolute;
        border-radius: 0.5rem;
        /* width: 10rem; */
        display: flex;
        flex-direction: column;
    }
</style>

<svelte:window on:contextmenu|preventDefault on:click={onLeftClick}/>