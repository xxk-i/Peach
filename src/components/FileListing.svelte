<script lang="ts">
    import { fileContextButtons, folderContextButtons } from "$lib/files";
	import { contextMenuInfo, folderPins } from "$lib/global";
    import { createEventDispatcher } from "svelte";
	import { path } from "@tauri-apps/api";

    export let directory: string;
    export let folders: string[];
    export let files: string[];

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
        let fullPath = await path.join(directory, folder);
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
                    $folderPins = [...$folderPins, directory];
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


<table class="select-none">
    <thead>
        <tr>
            <th scope="col">Name</th>
            <th scope="col">Date Modified</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <button class="text-left w-full" on:click={leaveDir}>..</button>
        </tr>
        {#each folders.sort() as folder}
            <tr>
                <th scope="row" class="font-normal">
                    <button class="text-left w-full" on:click={() => enterDir(folder)} on:contextmenu={() => setFolderContextMenu(folder)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">folder
                        </span>
                    {folder}</button>
                </th>
            </tr>
        {/each}
        {#each files.sort() as file}
            <tr>
                <th scope="row" class="font-normal">
                    <button class="text-left w-full" on:click={() => openFile(file)} on:contextmenu={() => setFileContextMenu(file)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">
                        draft
                        </span>
                    {file}</button>
                </th>
            </tr>
        {/each}
    </tbody>
</table>