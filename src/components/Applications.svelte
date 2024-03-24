<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { convertFileSrc } from "@tauri-apps/api/tauri";

    type InstalledApplication = {
        name: string,
        icon: string,
        path: string
    }

    let appsPromise: Promise<InstalledApplication[]> = invoke("get_applications");
</script>

{#await appsPromise}
{:then apps}
    {#each apps as app}
        <div class="overflow-auto">
            <img src={convertFileSrc(app.icon)} alt={app.name}/>
            <h1>{app.name}</h1>
        </div>
    {/each}
{/await}
