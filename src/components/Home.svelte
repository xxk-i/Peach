<script lang="ts">
	import { invoke } from "@tauri-apps/api";
    import { Spinner } from "flowbite-svelte";
    import type { MountedDrive } from "$lib/mounted_drive";
    import Drive from "../components/Drive.svelte"
	import type { DirectoryInfo } from "$lib/files";

    let drivesPromise: Promise<Array<MountedDrive>> = getDrives();
    let directoryInfoPromise: Promise<DirectoryInfo>;
    let directory = "";

    async function getDrives(): Promise<Array<MountedDrive>> {
        return invoke('get_drives')
    }

    function enterDrive(letter: string) {
        directory += letter + ":\\";
        directoryInfoPromise = invoke('get_files_at_path', {path: directory})
    }

    function enterFolder(folder: string) {
        directory += "\\" + folder;
        directoryInfoPromise = invoke('get_files_at_path', {path: directory})
    }

    function leaveFolder() {
        // If we are at root, leave to Home
        if (directory.endsWith(":\\")) {
            directory = ""
        } else {
            directory = directory.slice(0, directory.lastIndexOf("\\"));
            directoryInfoPromise = invoke('get_files_at_path', {path: directory})
        }
    }

    function openFile(path: string) {
        invoke('open_file', {path: directory + "\\" + path})
    }

    function getRowUtility(drives: Array<MountedDrive>) {
        return "grid-cols-" + Math.ceil(Math.sqrt(drives.length))
    }
</script>

<!-- File browser homepage, displays every browsable disk -->

{#if directory == ""}
    <div class="flex place-content-center size-full">
        {#await drivesPromise}
            <Spinner color="purple" />
        {:then drives}
            <div class="grid grid-flow-row place-content-center gap-y-5 gap-x-5 {getRowUtility(drives)}">
                {#each drives as drive}
                    <Drive info={drive} on:click={() => enterDrive(drive.letter)}/>
                {/each}
            </div>
        {/await}
    </div>
{:else}
    {#await directoryInfoPromise}
        <div class="flex place-content-center size-full">
            <Spinner color="purple" />
        </div>
    {:then info}
        <div class="grid grid-flow-row overflow-auto place-content-start">
            <button class="text-left" on:click={leaveFolder}>..</button>
            {#each info.folders.sort() as folder}
                <button class="text-left" on:click={() => enterFolder(folder)}>[F] {folder}</button>
            {/each}
            {#each info.files.sort() as file}
                <button class="text-left" on:click={() => openFile(file)}>[Fi] {file}</button>
            {/each}
        </div>
    {/await}
{/if}
