<script lang="ts">
    import { type ContextMenuButton } from "$lib/global";
    import { contextMenuInfo } from "$lib/global";
	import { window } from "@tauri-apps/api";
	import { Menu, MenuItem, Submenu } from "@tauri-apps/api/menu";

    async function doMenu() {
        let menu = await Menu.new();
        for (let button of $contextMenuInfo.buttons) {
            menu.append(await MenuItem.new({
                text: button.title,
                action: button.callback
            }));
        }

        menu.popup(undefined, window.getCurrent());
    }
</script>

<!-- IMPORTANT -->
<!-- Disable right click and enable left click capture -->
<svelte:window on:contextmenu|preventDefault on:contextmenu={doMenu}/>