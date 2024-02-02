<script lang="ts">
	import type { DirectoryInfo } from "$lib/files";
	import type { Writable } from "svelte/store";
    import { getContext, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { tabsInfo } from "$lib/tab";
    import { fileContextButtons } from "$lib/files";
	import { contextMenuInfo } from "$lib/global";

    export let info: DirectoryInfo;

    let dir: string;
    let ctx: Writable<string> = getContext("dir");
    
    const unsubscribe = ctx.subscribe((value) => { 
        dir = value
    });

    function setDir(dir: string, name: string) {
        ctx.set(dir);
        $tabsInfo.buttonInfos = [...$tabsInfo.buttonInfos.slice(0, $tabsInfo.selected), name, ...$tabsInfo.buttonInfos.slice($tabsInfo.selected + 1, $tabsInfo.buttonInfos.length)];
        // $tabsInfo.buttonInfos[$tabsInfo.selected] = name;
    }

    function updateDir(dir: string, name: string) {
        ctx.update((value) => value += dir);
        $tabsInfo.buttonInfos = [...$tabsInfo.buttonInfos.slice(0, $tabsInfo.selected), name, ...$tabsInfo.buttonInfos.slice($tabsInfo.selected + 1, $tabsInfo.buttonInfos.length)];
    }

    function enterFolder(folder: string) {
        updateDir("/" + folder, folder);
    }

    function leaveFolder() {
        // If we are at root, leave to Home
        if (dir.endsWith(":/")) {
            setDir("/Home/", "Home");
        } else {
            let newDir = dir.slice(0, dir.lastIndexOf("/"));
            let name = newDir.slice(newDir.lastIndexOf("/") + 1, newDir.length);
            if (name != "") {
                setDir(newDir, name);
            } else { // dir is back at disk root
                setDir(newDir, newDir);
            }
        }
    }

    function openFile(path: string) {
        invoke('open_file', {path: dir + "/" + path})
    }

    function setContextMenu(file: string) {
        $contextMenuInfo.buttons = fileContextButtons;
        $contextMenuInfo.isShowing = true;
    }
</script>

<div class="grid grid-flow-row place-content-start w-full">
    <button class="text-left" on:click={leaveFolder}>..</button>
    {#each info.folders.sort() as folder}
        <div class="flex flex-row gap-x-1 w-full">
            <span class="material-symbols-outlined">
            folder
            </span>
            <button class="text-left w-full" on:click={() => enterFolder(folder)}>{folder}</button>
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