<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";

    enum SortType {
        Name,
        Date
    }

    let sortType = SortType.Name;
    let renameFile = "";

    let columns = 2;
    export let directory: string;
    export let folders: string[];
    export let files: string[];

    onMount(() => {
        sort();
    });

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

    function sort() {
        // sort() modifies in place, but we have to do assignments
        // so we can trigger the Svelte reactivity
        switch (sortType) {
            case SortType.Name:
                folders = folders.sort();
                files = files.sort();
                break;
            case SortType.Date:
                folders = folders.sort((a, b) => b.localeCompare(a));
                files = files.sort();
                break;
        }
    }

    function dispatchFileRightClick(file: string) {
        dispatch('altclickFile', {
            file,
        });
    }

    function dispatchFolderRightClick(folder: string) {
        dispatch('altclickFolder', {
            folder
        });
    }

    export function startRename(file: string) {
        renameFile = file;
    }
</script>


<table class="select-none w-full">
    <thead>
        <tr>
            <th scope="col" class="font-normal select-none cursor-pointer" on:click={() => sortType = SortType.Name}>Name</th>
            <th scope="col" class="font-normal select-none cursor-pointer" on:click={() => sortType = SortType.Date}>Date Modified</th>
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
        {#each folders as folder}
            <tr class="odd:bg-peach-600">
                <th scope="row" class="font-normal">
                    <button class="text-left w-full" on:click={() => enterDir(folder)} on:contextmenu={() => dispatchFolderRightClick(folder)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">folder
                        </span>
                    {folder}</button>
                </th>
                <th scope="row" class="font-normal">
                    1/1/1970
                </th>
            </tr>
        {/each}
        {#each files as file (file)}
            <tr class="odd:bg-peach-600">
                <th scope="row" class="font-normal">
                    {#if file === renameFile}
                        <input class="text-left justify-left"> 
                    {:else}
                    <button class="text-left w-full" on:click={() => openFile(file)} on:contextmenu={() => dispatchFileRightClick(file)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">
                        draft
                        </span>
                    {file}</button>
                    {/if}
                </th>
                <!-- TODO actual file metadata -->
                <th scope="row" class="font-normal">
                    1/1/1970
                </th>
            </tr>
        {/each}
    </tbody>
</table>