<script lang="ts">
	import type { DirectoryInfo } from "$lib/files";
	import type { Writable } from "svelte/store";
    import { getContext, onDestroy } from "svelte";
    import { fileContextButtons } from "$lib/files";
	import { contextMenuInfo } from "$lib/global";
    import { createEventDispatcher } from "svelte";

    export let info: DirectoryInfo;

    const dispatch = createEventDispatcher();

    function enterDir(dir: string) {
        // updateDir("/" + folder, folder);
        dispatch('enterDir', {
            dir,
        });
    }

    function leaveDir() {
        // If we are at root, leave to Home
        // if (dir.endsWith(":/")) {
        //     setDir("/Home/", "Home");
        // } else {
        //     let newDir = dir.slice(0, dir.lastIndexOf("/"));
        //     let name = newDir.slice(newDir.lastIndexOf("/") + 1, newDir.length);
        //     if (name != "") {
        //         setDir(newDir, name);
        //     } else { // dir is back at disk root
        //         setDir(newDir, newDir);
        //     }
        // }
        dispatch('leaveDir');
    }

    function openFile(path: string) {
        dispatch('clickFile', {
            path,
        });
    }

    function setContextMenu(file: string) {
        $contextMenuInfo.buttons = fileContextButtons;
        $contextMenuInfo.isShowing = true;
    }
</script>

<div class="grid grid-flow-row place-content-start w-full">
    <button class="text-left" on:click={leaveDir}>..</button>
    {#each info.folders.sort() as folder}
        <div class="flex flex-row gap-x-1 w-full">
            <span class="material-symbols-outlined">
            folder
            </span>
            <button class="text-left w-full" on:click={() => enterDir(folder)}>{folder}</button>
        </div>
    {/each}
    {#each info.files.sort() as file}
        <div class="flex flex-row gap-x-1">
            <span class="material-symbols-outlined">
            draft
            </span>
            <button class="text-left" on:click={() => openFile(file)} on:contextmenu={() => setContextMenu(file)}>{file}</button>
        </div>
    {/each}
</div>