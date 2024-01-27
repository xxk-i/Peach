import { writable } from "svelte/store";

export type TabsInfo = {
    count: number,
    selected: number,
};

export const tabsInfo = writable<TabsInfo>({
    count: 1,
    selected: 0,
});