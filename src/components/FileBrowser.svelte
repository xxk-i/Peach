<script lang="ts">
    import { os_path } from "$lib";
	import { contextMenuInfo, folderPins } from "$lib/global";
	import type { DirectoryInfo } from "$lib/files";
	import type { TabInfo } from "$lib/stores";
	import type { MountedDrive } from "$lib/mounted_drive";
	import { type Writable } from "svelte/store";
    import { Progressbar, Spinner } from "flowbite-svelte";
	import { invoke } from "@tauri-apps/api";
	import { getContext, onDestroy } from "svelte";
    import FileListing from "./FileListing.svelte";
	import MiniSearchBar from "./MiniSearchBar.svelte";

    let filter = "";
    let diskSpaceInfoPromise: Promise<MountedDrive>;
    let dir = "";
    let tabName = "";

    let tabInfo: Writable<TabInfo> = getContext("tabInfo");

    const unsubscribe = tabInfo.subscribe((info) => { 
        if (!diskSpaceInfoPromise) {
            diskSpaceInfoPromise = invoke('get_disk_space_info', {disk: info.directory});
        }

        dir = info.directory;
        tabName = info.name;
    });

    function openFile(event: CustomEvent) {
        invoke('open_file', {path: dir + "/" + event.detail.path})
    }

    async function enterDir(event: CustomEvent) {
        let newPath = await os_path.push(dir, event.detail.dir);
        tabInfo.update((store) => ({
            id: store.id,
            name: event.detail.dir,
            directory: newPath
        }));
    }

    async function leaveDir(event: CustomEvent) {
        // If we are at root, leave to Home
        if (await os_path.pop(dir) === dir) {
            tabInfo.update((store) => ({
                id: store.id,
                name: "Home",
                directory: "/Home/"
            }))
        } else {
            let newDir = await os_path.pop(dir)
            let name = os_path.get_name(newDir);
            tabInfo.update((store) => ({
                id: store.id,
                name: newDir === "/" ? newDir : name,
                directory: newDir,
            }))
        }
    }

    function onEmptySpaceContextMenu(event: { target: HTMLDivElement | null; }) {
        if (event.target == null) {
            return
        }

        const target: HTMLDivElement = event.target;
        if (target.classList.contains("empty-space")) {
            showEmptySpaceContextMenu();
        }
    }

    function showEmptySpaceContextMenu() {
        $contextMenuInfo.buttons = [
            {
                title: "Pin Current Directory",
                callback: () => {
                    $folderPins = [...$folderPins, dir];
                }
            }
        ];
        $contextMenuInfo.isShowing = true;
    }

    let directoryInfoPromise: Promise<DirectoryInfo>;
    $: {
        directoryInfoPromise = invoke('get_files_at_path', {path: dir});
    }

    onDestroy(unsubscribe);
</script>

<div class="size-full flex flex-col">
    <!-- this ts error on event types is really annoying and maybe unfixable rn -->
    <div class="empty-space overflow-auto grow pb-5" on:contextmenu={onEmptySpaceContextMenu}>
        {#await directoryInfoPromise}
            <div class="flex place-content-center size-full">
                <Spinner color="purple" />
            </div>
        {:then info}
            <div class="float-right">
                <div class="absolute top-o right-0">
                    <MiniSearchBar {filter}/>
                </div>
            </div>
            <FileListing {info} {filter} on:clickFile={openFile} on:enterDir={enterDir} on:leaveDir={leaveDir}/>
        {/await}
    </div>
    <div class="ml-1 mr-1 mb-1">
        {#await diskSpaceInfoPromise}
            <div class="flex place-content-center size-full">
                <Spinner color="purple" />
            </div>
        {:then diskSpaceInfo}
            <Progressbar progress={Math.floor(diskSpaceInfo.storage_used / diskSpaceInfo.capacity * 100)} color="yellow" labelOutside={dir.slice(0,3)}/>
        {/await}
    </div>
</div>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<style>
.material-symbols-outlined {
  font-variation-settings:
  'FILL' 0,
  'wght' 400,
  'GRAD' 0,
  'opsz' 24
}
</style>