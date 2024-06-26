<script lang="ts">
    import { os_path } from "$lib";
	import { contextMenuInfo, folderPins } from "$lib/global";
	import type { DirectoryInfo } from "$lib/files";
	import { tabStore, type TabInfo } from "$lib/stores";
	import type { MountedDrive } from "$lib/mounted_drive";
	import { writable, type Writable } from "svelte/store";
    import { Progressbar, Spinner } from "flowbite-svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { getContext, onDestroy } from "svelte";
    import FileListing from "./FileListing.svelte";
	import MiniSearchBar from "./MiniSearchBar.svelte";
	import { path } from "@tauri-apps/api";

    // TODO maybe better way than store here?
    let filter = writable("");
    let diskSpaceInfoPromise: Promise<MountedDrive>;
    let dir = "";
    let tabName = "";

    let tabInfo: Writable<TabInfo> = getContext("tabInfo");
    let fileListing: FileListing;

    const unsubscribe = tabInfo.subscribe((info) => { 
        if (!diskSpaceInfoPromise) {
            diskSpaceInfoPromise = invoke('get_disk_space_info', {disk: info.path});
        }

        dir = info.path;
        tabName = info.name;
    });

    function openFile(file: string) {
        invoke('open_file', {path: dir + "/" + file})
    }

    async function enterDir(event: CustomEvent) {
        let newPath = await os_path.push(dir, event.detail.dir);
        tabStore.setSelectedToPath(newPath, event.detail.dir);
        $filter = "";
    }

    async function leaveDir(event: CustomEvent) {
        // If we are at root, leave to Home
        if (await os_path.pop(dir) === dir) {
            tabStore.setSelectedToHome();
        } else {
            let newPath = await os_path.pop(dir)
            let name = os_path.get_name(newPath);
            tabStore.setSelectedToPath(newPath, name);
        }
        $filter = "";
    }
    async function setFolderContextMenu(folder: string) {
        let fullPath = await path.join(dir, folder);
        $contextMenuInfo.buttons = [{
            title: "Pin Folder",
            callback: () => {
                $folderPins = [...$folderPins, fullPath];
            }
        },
        {
            title: "Pin Current Directory",
            callback: () => {
                $folderPins = [...$folderPins, dir];
            }
        },
        {
            title: "Sync Directory",
            callback: () => {
                tabStore.addSyncTab(fullPath);
            }
        }];
        $contextMenuInfo.isShowing = true;
    }

    function setFileContextMenu(file: string) {
        $contextMenuInfo.buttons = [{
            title: "Open File",
            callback: () => {
                openFile(file)
            }
        },
        {
            title: "Open with...",
            callback: () => {
                console.log("Open with clicked");
            }
        },
        {
            title: "Rename",
            callback: () => {
                fileListing.startRename(file)
            }
        }];
        $contextMenuInfo.isShowing = true;
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
                <div class="absolute right-1">
                    <MiniSearchBar {filter}/>
                </div>
            </div>
            <FileListing
                directory={info.path}
                folders={info.folders.filter((name) => name.toLowerCase().includes($filter.toLowerCase()))} 
                files={info.files.filter((name) => name.toLowerCase().includes($filter.toLowerCase()))} 
                bind:this={fileListing}
                on:clickFile={(event) => openFile(event.detail.file)}
                on:altclickFile={(event) => setFileContextMenu(event.detail.file)}
                on:altclickFolder={(event) => setFolderContextMenu(event.detail.folder)}
                on:enterDir={enterDir} 
                on:leaveDir={leaveDir}
            />
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