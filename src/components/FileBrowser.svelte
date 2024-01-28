<script lang="ts">
    import { Spinner } from "flowbite-svelte";
	import { invoke } from "@tauri-apps/api";
	import type { DirectoryInfo } from "$lib/files";
	import { getContext, onDestroy } from "svelte";
	import type { Writable } from "svelte/store";
    import { tabsInfo } from "$lib/tab";

    let dir: string;
    let ctx: Writable<string> = getContext("dir");

    const unsubscribe = ctx.subscribe((value) => dir = value);

    let directoryInfoPromise: Promise<DirectoryInfo>;
    $: directoryInfoPromise = invoke('get_files_at_path', {path: dir});

    function getItemsInDirectory() {
        directoryInfoPromise = invoke('get_files_at_path', {path: dir});
    }

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
        getItemsInDirectory();
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
            getItemsInDirectory();
        }
    }

    function openFile(path: string) {
        invoke('open_file', {path: dir + "/" + path})
    }

    onDestroy(unsubscribe);
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<div class="size-full overflow-auto">
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