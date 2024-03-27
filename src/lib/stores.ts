import { os_path } from "$lib";
import { get, writable, type Writable } from "svelte/store";

export type TabInfo = {
    name: string,
    id: number,
    directory: string
}

function createTabStore() {
    const { subscribe, set, update } = writable({infos: [writable({name: "Home", id: 0, directory: "/Home/"})], selected: 0, nextId: 1});

    function setSelected(index: number) {
        update((store) => ({
            ...store,
            selected: index
        }));
    }

    function setSelectedToDir(dir: string, name: string) {
        update((store) => {
            for (var info of store.infos) {
                if (get(info).id == store.selected) {
                    info.update((info) => ({
                        id: info.id,
                        name: name,
                        directory: dir
                    }));
                }
            }

            return store;
        });
    }

    function setSelectedToHome() {
        update((store) => {
            for (var info of store.infos) {
                if (get(info).id == store.selected) {
                    info.update((info) => ({
                        id: info.id,
                        name: "Home",
                        directory: "/Home/"
                    }));
                }
            }

            return store;
        });
    }
    
    function setSelectedToApps() {
        update((store) => {
            for (var info of store.infos) {
                if (get(info).id == store.selected) {
                    info.update((info) => ({
                        id: info.id,
                        name: "Apps",
                        directory: "/Applications/"
                    }));
                }
            }

            return store;
        });
    }

    function addTab() {
        update((store) => ({
            ...store,
            infos: [...store.infos, writable({name: "Home", id: store.nextId, directory: "/Home/"})],
            nextId: store.nextId + 1,
        }));
    }

    function addTabAndSelectIt() {
        update((store) => ({
            selected: store.nextId,
            infos: [...store.infos, writable({name: "Home", id: store.nextId, directory: "/Home/"})],
            nextId: store.nextId + 1,
        }));
    }

    function addTabAtPath(dir: string) {
        update((store) => ({
            selected: store.nextId,
            infos: [...store.infos, writable({name: os_path.get_name(dir), id: store.nextId, directory: dir})],
            nextId: store.nextId + 1
        }));
    }

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
        setSelectedToDir,
        setSelectedToHome,
        setSelectedToApps,
        addTab,
        addTabAndSelectIt,
        closeTab
    }
}

export let tabStore = createTabStore();