<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { appCacheDir } from "@tauri-apps/api/path";
	import { Spinner } from "flowbite-svelte";
	import { tabStore } from "$lib/stores";
	import { os_path } from "$lib";

    type InstalledApplication = {
        name: string,
        icon: string,
        path: string
    }

    let appCacheDirPromise: Promise<string> = appCacheDir();
    let appsPromise: Promise<InstalledApplication[]> = invoke("get_applications");
    
    async function navigateToAppDir(app: InstalledApplication) {
        console.log("hi");
        let name = await os_path.get_name(app.path);
        tabStore.setSelectedToDir(app.path, name);
    }

</script>

<!-- TODO fix all this await disaster -->
<div class="flex justify-between h-full overflow-auto flex-wrap">
    {#await appCacheDirPromise}
    <Spinner/>
    {:then cacheDir}
        {#await appsPromise}
        <Spinner/>
        {:then apps}
            {#each apps as app}
            <div class="flex flex-col text-center text-wrap p-5">
                <button on:click={async () => await navigateToAppDir(app)}>
                    <img class="m-auto max-w-20 min-w-20 text-center" src={convertFileSrc(cacheDir + "/icons/" + app.name + ".png")} alt={app.name}/>
                    <h1 class="text-wrap">{app.name}</h1>
                </button>
            </div>
            {/each}
        {/await}
    {/await}
</div>
