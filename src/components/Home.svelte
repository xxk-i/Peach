<script lang="ts">
    import { Spinner } from "flowbite-svelte";
	import { invoke } from "@tauri-apps/api";
    import type { MountedDrive } from "$lib/mounted_drive";
    import Drive from "../components/Drive.svelte"
	import { getContext } from "svelte";
	import { type Writable } from "svelte/store";
	import type { TabInfo } from "$lib/stores";

    let tabInfo: Writable<TabInfo> = getContext("tabInfo");

    let drivesPromise: Promise<Array<MountedDrive>> = getDrives();

    async function getDrives(): Promise<Array<MountedDrive>> {
        return invoke('get_drives')
    }

    // TODO fix windows
    function enterDrive(mount_point: string) {
        tabInfo.update((store) => ({
            ...store,
            directory: mount_point
        }));
    }

    function getRowUtility(drives: Array<MountedDrive>) {
        return "grid-cols-" + Math.ceil(Math.sqrt(drives.length))
    }
</script>

<!-- File browser homepage, displays every browsable disk -->
<div class="flex place-content-center size-full">
    {#await drivesPromise}
        <Spinner color="purple" />
    {:then drives}
        <div class="grid grid-flow-row place-content-center gap-y-5 gap-x-5 {getRowUtility(drives)}">
            {#each drives as drive}
                <Drive info={drive} on:click={() => enterDrive(drive.mount_point)}/>
            {/each}
        </div>
    {/await}
</div>
