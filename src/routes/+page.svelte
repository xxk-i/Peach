<script lang="ts">
    import "../app.css"

    import { getCurrent } from "@tauri-apps/api/webviewWindow";
    import { get } from "svelte/store";
    import { mousePosition } from "$lib/global";
    import { tabStore } from "$lib/stores";
    import logo from "$lib/assets/Peach.png";
	import Sidebar from "../components/Sidebar.svelte";
    import ContextMenu from "../components/ContextMenu.svelte";
    import MainView from "../components/MainView.svelte";
    import Tabs from "../components/Tabs.svelte";

    let showSideBar = true;
    let window = getCurrent();

    function flipShowSideBar() {
        showSideBar = !showSideBar;
    }
    
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

<div class="rounded-lg bg-peach-500 overflow-hidden h-screen flex flex-col" on:mousemove={handleMouseMove}>
    <div data-tauri-drag-region class="titlebar flex flex-initial flex-grow-0
    max-h-[30px] bg-[rgb(230,_73,_125)] select-none rounded-tl-[10px] rounded-tr-[10px]">
            <img
            class="ml-1 mr-3 max-h-full"
            src={logo}
            alt="Peach"
            on:click={flipShowSideBar}
            />
        <div data-tauri-drag-region class="hide-scrollbar flex gap-x-px mt-1 shrink basis-full grow-0 overflow-x-auto overflow-y-hidden text-nowrap">
            <Tabs/>
        </div>
        <div class="titlebar-button-container">
            <div class="titlebar-button" id="titlebar-minimize" on:click={window.minimize}>
                <img
                src="https://api.iconify.design/mdi:window-minimize.svg"
                alt="minimize"
                />
            </div>
            <div class="titlebar-button" id="titlebar-maximize" on:click={window.toggleMaximize}>
                <img
                src="https://api.iconify.design/mdi:window-maximize.svg"
                alt="maximize"
                />
            </div>
            <div class="titlebar-button" id="titlebar-close" on:click={window.close}>
                <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
            </div>
        </div>
    </div>

    <div class="flex flex-row grow">
        <div class="shrink-0 grow-0 pr-7 pl-1 h-0 min-h-full {showSideBar ? "" : "hidden"} overflow-auto">
            <Sidebar/>
        </div>

        {#each $tabStore.infos as info (get(info).id)}
            <MainView tabInfo={info}></MainView>
        {/each}
    </div>

    <ContextMenu/>
</div>

<style>
    .hide-scrollbar::-webkit-scrollbar {
        background: transparent; /* Chrome/Safari/Webkit */
        height: 0px;
    }

    .titlebar-button-container {
        display: flex;
        justify-content: flex-end;
        min-width: fit-content;
        flex-grow: 0;
        flex-shrink: 0;
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