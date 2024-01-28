import { writable } from "svelte/store";

export type TabsInfo = {
    buttonInfos: string[],
    selected: number,
};

export const tabsInfo = writable<TabsInfo>({
    buttonInfos: [],
    selected: 0,
});