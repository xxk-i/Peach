<script lang="ts">
    import "../app.css"
    import { appWindow } from "@tauri-apps/api/window";
    import logo from "$lib/assets/Peach.png";
	import Tabs from "../components/Tabs.svelte";
	import { invoke } from "@tauri-apps/api";

    function popupAboutWindow() {
        alert("hello")
    }

    function getVolumes() {
        invoke('get_volumes')
    }

    function getDrives() {
        invoke('get_drives')
    }
</script>

<div class="flex rounded-lg bg-[#e6497d] overflow-hidden h-screen">
    <div data-tauri-drag-region class="titlebar">
        <div data-tauri-drag-region class="icon-container">
            <img
            src={logo}
            alt="Peach"
            on:click={popupAboutWindow}
            />
        </div>
        <div class="flex gap-x-px mt-1">
            <Tabs/>
        </div>
        <div data-tauri-drag-region class="titlebar-button-container">
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
    <div class="flex mt-10 grow-0">
        <h1>hello!!</h1>
        <h1>hello!!</h1>
        <h1>hello!!</h1>
    </div>
    <button on:click={getVolumes}>
        get volumes
    </button>
    <button on:click={getDrives}>
        get drives
    </button>
</div>

<style>
    .icon-container {
        display: flex;
        justify-content: flex-start;
        margin-left: 5px;
        margin-top: 1px;
        min-width: max-content;
        max-width: max-content;
        margin-right: 30px;
        width: 100%;
    }

    .titlebar-button-container {
        display: flex;
        justify-content: flex-end;
        width: 100%;
    }

    .titlebar {
    display: flex;
    flex-direction: row;
    height: 30px;
    background: rgb(230, 73, 125);
    user-select: none;
    border-top-left-radius: 10px;
    border-top-right-radius: 10px;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
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