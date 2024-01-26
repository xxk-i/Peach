<script lang="ts">
    import { Spinner } from "flowbite-svelte";
	import { invoke } from "@tauri-apps/api";
	import type { DirectoryInfo } from "$lib/files";
	import { tabsCache } from "$lib/tab";

    let directoryInfoPromise: Promise<DirectoryInfo>;
    $: directoryInfoPromise = invoke('get_files_at_path', {path: $tabsCache.tabs[$tabsCache.selected].dir});

    function getItemsInDirectory() {
        directoryInfoPromise = invoke('get_files_at_path', {path: $tabsCache.tabs[$tabsCache.selected].dir});
    }

    function setDir(dir: string, name: string) {
        $tabsCache.tabs[$tabsCache.selected].dir = dir;
        $tabsCache.tabs[$tabsCache.selected].name = name;
    }

    function updateDir(dir: string, name: string) {
        $tabsCache.tabs[$tabsCache.selected].dir += dir;
        $tabsCache.tabs[$tabsCache.selected].name = name;
    }

    function enterFolder(folder: string) {
        updateDir("/" + folder, folder);
        getItemsInDirectory();
    }

    function leaveFolder() {
        // If we are at root, leave to Home
        if ($tabsCache.tabs[$tabsCache.selected].dir.endsWith(":/")) {
            setDir("/Home/", "Home");
        } else {
            let oldDir = $tabsCache.tabs[$tabsCache.selected].dir;
            let newDir = oldDir.slice(0, oldDir.lastIndexOf("/"));
            let name = newDir.slice(newDir.lastIndexOf("/") + 1, newDir.length);
            if (name != "") {
                setDir(newDir, name);
            } else { // dir is back at disk root
                setDir(newDir, newDir);
            }
            getItemsInDirectory();
        }
    }

    function openFile(path: string) {
        invoke('open_file', {path: $tabsCache.tabs[$tabsCache.selected].dir + "/" + path})
    }
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<div class="grow shrink basis-auto overflow-auto">
    {#await directoryInfoPromise}
        <div class="flex place-content-center size-full">
            <Spinner color="purple" />
        </div>
    {:then info}
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
                    <button class="text-left" on:click={() => openFile(file)}>{file}</button>
                </div>
            {/each}
        </div>
    {/await}
</div>
<style>
.material-symbols-outlined {
  font-variation-settings:
  'FILL' 0,
  'wght' 400,
  'GRAD' 0,
  'opsz' 24
}
</style>