<script lang="ts">
    import "../app.css"
    import { appWindow } from "@tauri-apps/api/window";
    import logo from "$lib/assets/Peach.png";
    import Tabs from "../components/Tabs.svelte";
    import { mousePosition } from "$lib/global";
    import ContextMenu from "../components/ContextMenu.svelte";
	import MainView from "../components/MainView.svelte";
	import { get } from "svelte/store";
    import { tabStore } from "$lib/stores";

    function popupAboutWindow() {
        alert("hello");
    }

    function handleMouseMove(event: MouseEvent ) {
        $mousePosition = {
            x: event.clientX,
            y: event.clientY,
        };
    }

</script>

<div class="rounded-lg bg-[#e6497d] overflow-hidden h-screen flex flex-col" on:mousemove={handleMouseMove}>
    <div data-tauri-drag-region class="titlebar">
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <div class="grow-1 shrink-0">
                <img
                class="ml-1 mr-3 max-h-full max-w-full"
                src={logo}
                alt="Peach"
                on:click={popupAboutWindow}
                />
            </div>
        <div data-tauri-drag-region class="flex gap-x-px mt-1 w-full shrink">
            <Tabs/>
        </div>
        <div class="titlebar-button-container">
            <div class="titlebar-button" id="titlebar-minimize" on:click={appWindow.minimize}>
                <img
                src="https://api.iconify.design/mdi:window-minimize.svg"
                alt="minimize"
                />
            </div>
            <div class="titlebar-button" id="titlebar-maximize" on:click={appWindow.toggleMaximize}>
                <img
                src="https://api.iconify.design/mdi:window-maximize.svg"
                alt="maximize"
                />
            </div>
            <div class="titlebar-button" id="titlebar-close" on:click={appWindow.close}>
                <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
            </div>
        </div>
    </div>

    <div class="grow min-h-0">
        {#each $tabStore.infos as info (get(info).id)}
            <MainView tabInfo={info}></MainView>
        {/each}
    </div>

    <!-- <div class="content_container grow min-h-0" bind:this={contentContainer}/> -->
    <ContextMenu/>
</div>

<style>
    .titlebar-button-container {
        display: flex;
        justify-content: flex-end;
        min-width: min-content;
        flex-grow: 0;
    }

    .titlebar {
    display: flex;
    flex-direction: row;
    flex: 0 1 auto;
    height: 30px;
    background: rgb(230, 73, 125);
    user-select: none;
    border-top-left-radius: 10px;
    border-top-right-radius: 10px;
    top: 0;
    left: 0;
    right: 0;
    margin-top: 1;
    }
    .titlebar-button {
    display: inline-flex;
    min-width: max-content;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    }
    .titlebar-button:hover {
    background: #5bbec3;
    }
</style>