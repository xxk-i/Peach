<script lang="ts">
	import { os_path } from "$lib";
	import { folderPins } from "$lib/global";
	import { get_name } from "$lib/os_path";
	import { tabStore } from "$lib/stores";
	import { path } from "@tauri-apps/api";

    function goHome() {
        tabStore.setSelectedToHome();
    }

    async function goUserHome() {
        let userHome = await path.join(await path.desktopDir(), ".."); // desktop parent is user $HOME lol
        let name = await os_path.get_name(userHome);
        tabStore.setSelectedToDir(userHome, name);
    }

    function goApps() {
        tabStore.setSelectedToApps();
    }
</script>

<nav class="select-none">
    <ul>
        <!-- <div class="flex flex-row gap-x-1 w-full"> -->
        <li>
            <br/>
            <!-- intentional gap -->
        </li>
        <li>
            <button on:click={goUserHome}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">person</span>
            User</button>
        </li>
        <li>
            <button on:click={goHome}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">cottage</span>
            Home</button>
        </li>
        <li>
            <button on:click={goApps}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">apps</span>
            Apps</button>
        </li>
        <li>
            <div class="text-center">-- Pins --</div>
        </li>
        <li>
            <button on:click={() => location.reload()}>
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">refresh</span>
            Refresh</button>
        </li>
        {#each $folderPins as folder}
            <li>
                {#await os_path.get_name(folder)}
                <span class="material-symbols-outlined" style="top: 5px; position: relative;">refresh</span>
                {:then name}
                    <button on:click={() => tabStore.setSelectedToDir(folder, name)}>
                        <span class="material-symbols-outlined" style="top: 5px; position: relative;">folder</span>
                    {name}</button>
                {/await}
            </li>
        {/each}
    </ul>
</nav>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />