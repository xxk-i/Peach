<script lang="ts">
	import MainView from "./MainView.svelte";
	import { writable } from "svelte/store";
    import { onMount } from "svelte";
	import Tab from "./Tab.svelte";
    import { tabsInfo } from "$lib/tab";
    import { contextMenuInfo } from "$lib/global";

    export let contentContainer: HTMLElement;

    onMount(() => {
        new MainView({
            target: contentContainer,
            props: {
                dir: writable("/Home/"),
            }
        });

        $tabsInfo.buttonInfos = ["Home"];
    });

    function swapToTab(index: number) {
        setViewToHidden($tabsInfo.selected);
        setViewToVisible(index);
        $tabsInfo.selected = index;
    }

    function setViewToHidden(index: number) {
        let mainViews = contentContainer.getElementsByClassName("mainview");
        (mainViews[index] as HTMLElement).style.display = "none";
    }

    function setViewToVisible(index: number) {
        let mainViews = contentContainer.getElementsByClassName("mainview");
        (mainViews[index] as HTMLElement).style.display = "";
    }
    
    function addTab() {
        new MainView({
            target: contentContainer,
            props: {
                dir: writable("/Home/"),
            }
        });
        $tabsInfo.buttonInfos = [...$tabsInfo.buttonInfos, "Home"];
        swapToTab($tabsInfo.buttonInfos.length - 1);
    }

    function handleAuxClick(event: MouseEvent, index: number) {
        if (event.button == 1) {
            closeTab(index);
        }
    }
    
    function closeTab(index: number) {
        if ($tabsInfo.buttonInfos.length == 1) {
            return;
        }

        let mainView = contentContainer.getElementsByClassName("mainview")[index];
        contentContainer.removeChild(mainView);

        $tabsInfo.buttonInfos = [...$tabsInfo.buttonInfos.slice(0, index), ...$tabsInfo.buttonInfos.slice(index + 1, $tabsInfo.buttonInfos.length)];
        if ($tabsInfo.selected >= index) {
            $tabsInfo.selected -= 1;
            setViewToVisible($tabsInfo.selected);
        }
    }


    function moveTab(from: number, to: number) {
        if ($tabsInfo.selected == from) {
            $tabsInfo.selected = to;
        }

        else if ($tabsInfo.selected == to) {
            $tabsInfo.selected = from;
        }

        let old = contentContainer.replaceChild(contentContainer.children[2], contentContainer.children[0]);
        contentContainer.appendChild(old);
        contentContainer = contentContainer;
        let oldName = $tabsInfo.buttonInfos[0];
        $tabsInfo.buttonInfos[0] = $tabsInfo.buttonInfos[2];
        $tabsInfo.buttonInfos[2] = oldName;
        $tabsInfo.buttonInfos = $tabsInfo.buttonInfos;
    }
    
    function setContextMenu(index: number) {
        $contextMenuInfo.buttons = 
        [
            {
                title: "Close Tab",
                callback: () => { closeTab(index) }
            }
        ];

        $contextMenuInfo.isShowing = true;
    }
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />

{#each $tabsInfo.buttonInfos as name, i}
    <Tab open={$tabsInfo.selected == i} name={name} on:click={() => swapToTab(i)} on:auxclick={(event) => handleAuxClick(event, i)} on:contextmenu={() => setContextMenu(i)}/>
{/each}

<button class="border-t border-l border-r rounded-t" on:click={addTab}>
    <span class="material-symbols-outlined">
    add
    </span>
</button>

<!-- 
<button on:click={() => moveTab(2, 0)}>
    swap
</button> -->