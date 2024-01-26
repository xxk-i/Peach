import { writable } from "svelte/store";

export type TabInfo = {
    name: string;
    dir: string;
};

export type TabsCache = {
    selected: number;
    tabs: TabInfo[];
}

export const tabsCache = writable<TabsCache>({
    selected: 0,
    tabs: [{
        name: "Home",
        dir: "/Home/",
    }],
});