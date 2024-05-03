<script lang="ts">
    import { fileContextButtons, folderContextButtons } from "$lib/files";
	import { contextMenuInfo, folderPins } from "$lib/global";
    import { createEventDispatcher } from "svelte";
	import { path } from "@tauri-apps/api";

    let columns = 2;
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


<table class="select-none w-full">
    <thead>
        <tr>
            <th scope="col" class="font-normal">Name</th>
            <th scope="col" class="font-normal">Date Modified</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <!-- Fun fact: there is no way to have colspan automatically span all columns -->
            <!-- Rumors of colspan="0" were either always wrong or unsupported from HTML5 -->
            <th scope="row" colspan="{columns}" class="font-normal bg-peach-600">
                <!-- Leave directory button -->
                <!-- TODO: move this somewhere that's always accessible (position: sticky had issues) -->
                <button class="text-left w-full" on:click={leaveDir}>..</button>
            </th>
        </tr>
        {#each folders.sort() as folder}
            <tr class="odd:bg-peach-600">
                <th scope="row" class="font-normal">
                    <button class="text-left w-full" on:click={() => enterDir(folder)} on:contextmenu={() => setFolderContextMenu(folder)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">folder
                        </span>
                    {folder}</button>
                </th>
                <th scope="row" class="font-normal">
                    1/1/1970
                </th>
            </tr>
        {/each}
        {#each files.sort() as file}
            <tr class="odd:bg-peach-600">
                <th scope="row" class="font-normal">
                    <button class="text-left w-full" on:click={() => openFile(file)} on:contextmenu={() => setFileContextMenu(file)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">
                        draft
                        </span>
                    {file}</button>
                </th>
                <!-- TODO actual file metadata -->
                <th scope="row" class="font-normal">
                    1/1/1970
                </th>
            </tr>
        {/each}
    </tbody>
</table>