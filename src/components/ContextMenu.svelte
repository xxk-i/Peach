<script lang="ts">
    import { mousePosition, type ContextMenuButton } from "$lib/global";
    import { contextMenuInfo } from "$lib/global";
	import { window } from "@tauri-apps/api";
	import { Menu, Submenu } from "@tauri-apps/api/menu";
	import { onDestroy } from "svelte";

    let left = 0;
    let top = 0;
    let isShowing = false;
    let buttons: ContextMenuButton[] = [];


    async function doMenu() {
        let menu = await Menu.new();
        let submenu = await Submenu.new({
            text: "Test"
        });

        menu.append(submenu);
        menu.popup(undefined, window.getCurrent());
    }

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
    <div class="absolute flex flex-col rounded-lg border-solid border-peach-300 border-2 bg-peach-400 p-1" style="left: {left + "px"}; top: {top + "px"};">
        {#each buttons as button}
            <button class="text-left" on:click={() => button.callback()}>{button.title}</button>
        {/each}
    </div>
{/if}

<!-- IMPORTANT -->
<!-- Disable right click and enable left click capture -->
<svelte:window on:contextmenu|preventDefault on:contextmenu={doMenu} on:click={onLeftClick}/>