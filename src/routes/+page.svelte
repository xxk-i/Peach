<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import Tab from "../components/Tab.svelte";
    import logo from "$lib/assets/Peach.png";

    // function windowMaximize() {
    //     if appWindow.isMaximized {
    //         appWindow.max
    //     }
    // }

    function popupAboutWindow() {
        alert("hello")
    }
</script>

<div class="rounded">
    <div data-tauri-drag-region class="titlebar">
        <div data-tauri-drag-region class="icon-container">
            <img
            src={logo}
            alt="Peach"
            on:click={popupAboutWindow}
            />
        </div>
        <div class="tab-container">
            <Tab/>
            <Tab/>
            <Tab/>
            <Tab/>
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
    <div class="main-view">
        <h1>hello!!</h1>
        <h1>hello!!</h1>
        <h1>hello!!</h1>
    </div>
</div>

<style>
    .main-view {
        /* display: flex; */
        margin-top: 5px;
        flex-grow: 1;
    }

    .tab-container {
        display: flex;
    }

    .rounded {
        display: flex;
        border-radius: 10px;
        background-color: rgb(230, 73, 125);
        height: 100vh;
        overflow: hidden;
    }

    .icon-container {
        display: flex;
        justify-content: flex-start;
        margin-left: 5px;
        margin-top: 1px;
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
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    }
    .titlebar-button:hover {
    background: #5bbec3;
    }
</style>