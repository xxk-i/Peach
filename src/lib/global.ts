import { writable } from "svelte/store";

export type MainViewInfo = {
    id: number;
    dir: string;
    name: string;
}

export let globalContext = writable<MainViewInfo[]>();

// Mouse
export type MousePosition = {
    x: number,
    y: number,
}

export let mousePosition = writable<MousePosition>();
// Mouse

// Context Menu
export type ContextMenuButton = {
    title: string,
    callback: () => any,
}

export type ContextMenuInfo = {
    buttons: ContextMenuButton[],
    isShowing: boolean,
}

export let contextMenuInfo = writable<ContextMenuInfo>({
    buttons: [],
    isShowing: false,
});
// Context Menu

// Sidebar
export let folderPins = writable<string[]>([]);
// Sidebar