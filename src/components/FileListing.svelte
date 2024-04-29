<script lang="ts">
	import type { DirectoryInfo } from "$lib/files";
    import { fileContextButtons, folderContextButtons } from "$lib/files";
	import { contextMenuInfo, folderPins } from "$lib/global";
    import { createEventDispatcher } from "svelte";
	import { path } from "@tauri-apps/api";

    export let info: DirectoryInfo;
    export let filter: string;

    const dispatch = createEventDispatcher();

    function enterDir(dir: string) {
        dispatch('enterDir', {
            dir,
        });
    }

    function leaveDir() {
        dispatch('leaveDir');
    }

    function openFile(path: string) {
        dispatch('clickFile', {
            path,
        });
    }

    async function setFolderContextMenu(folder: string) {
        let fullPath = await path.join(info.path, folder);
        $contextMenuInfo.buttons = [
            {
                title: "Pin Folder",
                callback: () => {
                    $folderPins = [...$folderPins, fullPath];
                }
            },
            {
                title: "Pin Current Directory",
                callback: () => {
                    $folderPins = [...$folderPins, info.path];
                }
            }
        ];
        $contextMenuInfo.isShowing = true;
    }

    function setFileContextMenu(file: string) {
        $contextMenuInfo.buttons = fileContextButtons;
        $contextMenuInfo.isShowing = true;
    }
</script>

<ul class="select-none">
    <li>
        <button class="text-left w-full" on:click={leaveDir}>..</button>
    </li>
    {#each info.folders.filter((name) => name.toLowerCase().includes(filter.toLowerCase())).sort() as folder}
        <li>
            <button class="text-left w-full" on:click={() => enterDir(folder)} on:contextmenu={() => setFolderContextMenu(folder)}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">folder
                </span>
            {folder}</button>
        </li>
    {/each}
    {#each info.files.filter((name) => name.toLowerCase().includes(filter.toLowerCase())).sort() as file}
        <li>
            <button class="text-left" on:click={() => openFile(file)} on:contextmenu={() => setFileContextMenu(file)}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">
                draft
                </span>
            {file}</button>
        </li>
    {/each}
</ul>