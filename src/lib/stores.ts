import { os_path } from "$lib";
import { get, writable } from "svelte/store";

/**
 * Special views (not the file browsing view) are denoted by a
 * recognized directory path
 */
export enum SpecialPath {
    Home = "/Home/",
    Applications = "/Applications",
    Sync = "/Sync/"
}

export type TabInfo = {
    name: string,
    id: number,
    path: string
}

/**
 * Each Tab gets a TabInfo managed through its TabStore
 * 
 * @returns A collection of methods to interact with the TabInfo
 */
function createTabStore() {
    const { subscribe, set, update } = writable({infos: [writable({name: "Home", id: 0, path: "/Home/"})], selected: 0, nextId: 1});

    function setSelected(index: number) {
        update((store) => ({
            ...store,
            selected: index
        }));
    }

    function setSelectedToPath(path: string, name: string) {
        update((store) => {
            for (var info of store.infos) {
                if (get(info).id == store.selected) {
                    info.update((info) => ({
                        id: info.id,
                        name: name,
                        path
                    }));
                }
            }

            return store;
        });
    }

    /**
     * Set the current tab to the special "Home" view,
     * currently under "/Home/"
     * 
     */
    function setSelectedToHome() {
        setSelectedToPath("/Home/", "Home");
    }
    
    /**
     * Set the current tab to the special "Applications" view,
     * currently under "/Applications"
     * 
     * @remarks
     * By using /Applications as the specialized dir, when a user
     * manually navigates to the default Applications folder manually on macOS,
     * this view will be shown instead
     */
    function setSelectedToApps() {
        setSelectedToPath("/Applications", "Apps");
    }

    /**
     * Set the current tab to the special "Sync" view,
     * currently under "/Sync/"
     */
    function setSelectedToSync() {
        update((store) => {
            for (var info of store.infos) {
                if (get(info).id == store.selected) {
                    info.update((info) => ({
                        id: info.id,
                        name: "Sync",
                        path: "/Sync/"
                    }));
                }
            }

            return store;
        });
    }

    /**
     * Adds a default tab without selecting it
     */
    function addTab() {
        update((store) => ({
            ...store,
            infos: [...store.infos, writable({name: "Home", id: store.nextId, directory: "/Home/"})],
            nextId: store.nextId + 1,
        }));
    }

    /**
     * Adds a default tab and selects it (default of clicking the tab plus button)
     */
    function addTabAndSelectIt() {
        update((store) => ({
            selected: store.nextId,
            infos: [...store.infos, writable({name: "Home", id: store.nextId, directory: "/Home/"})],
            nextId: store.nextId + 1,
        }));
    }

    /**
     * Adds a tab at the given dir without selecting it 
     * @param path - The path for the tab to start in
     */
    function addTabAtPath(path: string) {
        update((store) => ({
            selected: store.nextId,
            infos: [...store.infos, writable({name: os_path.get_name(path), id: store.nextId, path})],
            nextId: store.nextId + 1
        }));
    }

    /**
     * Closes a tab of the given id 
     * @param id - The tab to be closed
     * 
     * @remarks
     * closeTab will also manage reselecting another tab,
     * should the currently selected tab be closed
     */
    function closeTab(id: number) {
        update((store) => {
            if (store.infos.length == 0) {
                return store;
            }

            let index = store.infos.findIndex((element) => get(element).id == id);
            let newSelection: number = store.selected;

            // if we close the current tab, switch selected to
            // previous (or next if we are the first tab)
            if (id == store.selected) {
                if (store.selected == get(store.infos[0]).id) {
                    newSelection = get(store.infos[1]).id;
                } else {
                    newSelection = get(store.infos[index - 1]).id;
                }
            }


            return {
                infos: [...store.infos.slice(0, index), ...store.infos.slice(index + 1)],
                selected: newSelection,
                nextId: store.nextId
            }
        });
    }

    return {
        subscribe,
        setSelected,
        setSelectedToPath,
        setSelectedToHome,
        setSelectedToApps,
        setSelectedToSync,
        addTab,
        addTabAndSelectIt,
        addTabAtPath,
        closeTab
    }
}

export let tabStore = createTabStore();