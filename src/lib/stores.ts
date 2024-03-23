import { get, writable, type Writable } from "svelte/store";

export type TabInfo = {
    name: string,
    id: number,
    directory: string
}

type Tabs = {
    infos: Writable<TabInfo>[],
    selected: number
}

function createTabStore() {
    const { subscribe, set, update } = writable({infos: [writable({name: "Home", id: 0, directory: "/Home/"})], selected: 0, nextId: 1});

    function setSelected(index: number) {
        update((store) => ({
            ...store,
            selected: index
        }));
    }

    function addTab() {
        update((store) => ({
            ...store,
            infos: [...store.infos, writable({name: "Home", id: store.nextId, directory: "/Home/"})],
            nextId: store.nextId + 1,
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
        addTab,
        closeTab
    }
}

export let tabStore = createTabStore();