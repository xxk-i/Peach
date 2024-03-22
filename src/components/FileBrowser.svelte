<script lang="ts">
    import { Progressbar, Spinner } from "flowbite-svelte";
	import { invoke } from "@tauri-apps/api";
	import { getContext, onDestroy } from "svelte";
    import FileListing from "./FileListing.svelte";
	import type { DirectoryInfo } from "$lib/files";
	import type { Writable } from "svelte/store";
	import type { MountedDrive } from "$lib/mounted_drive";

    let dir: string;
    let diskSpaceInfoPromise: Promise<MountedDrive>;
    let ctx: Writable<string> = getContext("dir");

    const unsubscribe = ctx.subscribe((value) => { 
        dir = value
        if (!diskSpaceInfoPromise) {
            diskSpaceInfoPromise = invoke('get_disk_space_info', {disk: dir[0]});
        }
    });

    function openFile(event: CustomEvent) {
        invoke('open_file', {path: dir + "/" + event.detail.path})
    }

    function enterDir(event: CustomEvent) {
        dir = dir + "/" + event.detail.dir;
    }

    function leaveDir(event: CustomEvent) {
        // If we are at root, leave to Home
        if (dir.endsWith(":/")) {
            dir = "/Home/"
            ctx.set("/Home/");
        } else {
            let newDir = dir.slice(0, dir.lastIndexOf("/"));
            let name = newDir.slice(newDir.lastIndexOf("/") + 1, newDir.length);
            if (name != "") {
                dir = newDir;
            } else { // dir is back at disk root
                dir = newDir;
            }
        }
    }

    let directoryInfoPromise: Promise<DirectoryInfo>;
    $: {
        if (dir != "/Home/") {
            directoryInfoPromise = invoke('get_files_at_path', {path: dir});
        }
    }

    onDestroy(unsubscribe);
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<div class="size-full flex flex-col">
    <div class="overflow-auto grow">
        {#await directoryInfoPromise}
            <div class="flex place-content-center size-full">
                <Spinner color="purple" />
            </div>
        {:then info}
            <FileListing info={info} on:clickFile={openFile} on:enterDir={enterDir} on:leaveDir={leaveDir}/>
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
<style>
.material-symbols-outlined {
  font-variation-settings:
  'FILL' 0,
  'wght' 400,
  'GRAD' 0,
  'opsz' 24
}
</style>