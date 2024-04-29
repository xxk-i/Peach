<script lang="ts">
	import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import { appCacheDir } from "@tauri-apps/api/path";
	import { Spinner } from "flowbite-svelte";
	import { tabStore } from "$lib/stores";
	import { contextMenuInfo } from "$lib/global";
	import { Command } from "@tauri-apps/api/shell";

    type InstalledApplication = {
        name: string,
        icon: string,
        path: string
    }

    let appCacheDirPromise: Promise<string> = appCacheDir();
    // TODO catch result
    let appsPromise: Promise<InstalledApplication[]> = invoke("get_applications");

    async function launchApp(app: InstalledApplication) { 
        const command = new Command("open-macos-app", ["-a", app.name]);
        await command.spawn();
    }

    function showApplicationContextMenu(app: InstalledApplication) {
        $contextMenuInfo.buttons = [
            {
                title: "Open",
                callback: () => {
                    launchApp(app);
                }
            },
            {
                title: "Open Application Folder",
                callback: () => {
                    tabStore.setSelectedToDir(app.path, app.name);
                }
            }
        ];
        $contextMenuInfo.isShowing = true;
    }

</script>

<!-- TODO fix all this await disaster -->
<div class="flex justify-between flex-wrap select-none">
    {#await appCacheDirPromise}
    <Spinner/>
    {:then cacheDir}
        {#await appsPromise}
        <Spinner/>
        {:then apps}
            {#each apps.sort(function (a, b) {
                if (a.name < b.name) {
                    return -1;
                }
                if (a.name > b.name) {
                    return 1;
                }
                return 0;
            }) as app}
            <div class="flex flex-col text-center text-wrap min-w-40 py-5">
                <button on:click={async () => await launchApp(app)} on:contextmenu={() => showApplicationContextMenu(app)}>
                    <img class="m-auto max-w-20 min-w-20 text-center" src={convertFileSrc(cacheDir + "/icons/" + app.name + ".png")} alt={app.name}/>
                    <h1 class="text-wrap">{app.name}</h1>
                </button>
            </div>
            {/each}
        {/await}
    {/await}
</div>
