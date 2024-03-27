<script lang="ts">
	import { tabStore } from "$lib/stores";

    function goHome() {
        tabStore.setSelectedToHome();
    }

    function goApps() {
        tabStore.setSelectedToApps();
    }
</script>

<nav>
    <ul>
        <!-- <div class="flex flex-row gap-x-1 w-full"> -->
        <li>
            <br/>
            <!-- intentional gap -->
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
        <!-- </div> -->
    </ul>
</nav>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />