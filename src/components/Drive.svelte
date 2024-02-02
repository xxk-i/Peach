<script lang="ts">
    import { Progressbar } from "flowbite-svelte";
    import type { MountedDrive } from "$lib/mounted_drive";

    export let info: MountedDrive;
</script>

<div class="flex flex-col text-yellow border border-yellow-200 rounded p-[5px] select-none" on:click>
        <div class="mb-1 text-base font-medium text-yellow-200">Drive {info.letter}</div>
        <Progressbar progress={Math.floor(info.storage_used / info.capacity * 100)} color="yellow"/>
        <div class="mb-1 text-base font-medium text-yellow-200">{info.capacity - info.storage_used} GB free of {info.capacity} GB</div>
</div>