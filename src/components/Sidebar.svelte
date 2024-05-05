<script lang="ts">
	import { os_path } from "$lib";
	import { contextMenuInfo, folderPins } from "$lib/global";
	import { tabStore } from "$lib/stores";
	import { path } from "@tauri-apps/api";
	import { invoke } from "@tauri-apps/api/core";
	import { BaseDirectory, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

    function goHome() {
        tabStore.setSelectedToHome();
    }

    async function openDevTools() {
        await invoke('open_dev_tools');
    }

    async function goUserHome() {
        let userHome = await path.join(await path.desktopDir(), ".."); // desktop parent is user $HOME lol
        let name = await os_path.get_name(userHome);
        tabStore.setSelectedToPath(userHome, name);
    }

    function goApps() {
        tabStore.setSelectedToApps();
    }

    function goSync() {
        tabStore.setSelectedToSync();
    }

    async function loadPins() {
        let text: string | null = null;
        try {
            text = await readTextFile('pins.json', { baseDir: BaseDirectory.AppConfig });
        } catch(e) {

        }
        
        if (text != null) {
            let pinList: string[] = JSON.parse(text);
            if (pinList != undefined) {
                $folderPins = pinList;
            }
        }
    }

    async function savePins() {
        try {
            await mkDir("", { dir: BaseDirectory.AppConfig });
        } catch(e) {
        }
        await writeTextFile('pins.json', JSON.stringify($folderPins), { baseDir: BaseDirectory.AppConfig });
    }

    function clearPins() {
        $folderPins = [];
    }

    function showPinnedFolderContextMenu(folder: string) {
        $contextMenuInfo.buttons = [
            {
                title: "Unpin",
                callback: () => {
                    let idx = $folderPins.findIndex(pin => pin == folder);
                    $folderPins.splice(idx, 1);
                    $folderPins = $folderPins;
                }
            }
        ];
        $contextMenuInfo.isShowing = true;
    }
</script>

<nav class="select-none">
    <ul>
        <li>
            <!-- intentional gap -->
            <br/>
        </li>
        <li>
            <button on:click={goUserHome}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">person</span>
            User</button>
        </li>
        <li>
            <button on:click={goHome}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">cottage</span>
            Home</button>
        </li>
        <li>
            <button on:click={goApps}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">apps</span>
            Apps</button>
        </li>
        <li>
            <button on:click={goSync}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">refresh</span>
            Sync</button>
        </li>
        <li>
            <div class="text-center">-- Pins --</div>
        </li>
        <li>
            <button on:click={savePins}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">save</span>
            Save Pins</button>
        </li>
        <li>
            <button on:click={loadPins}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">upload</span>
            Load Pins</button>
        </li>
        <li>
            <button on:click={clearPins}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">clear_all</span>
            Clear Pins</button>
        </li>
        {#each $folderPins as folder}
            <li>
                {#await os_path.get_name(folder)}
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">refresh</span>
                {:then name}
                    <button on:click={() => tabStore.setSelectedToPath(folder, name)} on:contextmenu={() => showPinnedFolderContextMenu(folder)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">folder</span>
                    {name}</button>
                {/await}
            </li>
        {/each}
        <li>
            <div class="text-center">-- Other --</div>
        </li>
        <li>
            <button on:click={() => location.reload()}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">refresh</span>
            Refresh</button>
        </li>
        <li>
            <button on:click={async () => await openDevTools()}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">code</span>
            Developer</button>
        </li>
        <li>
            <!-- intentional gap -->
            <br/>
        </li>
    </ul>
</nav>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />