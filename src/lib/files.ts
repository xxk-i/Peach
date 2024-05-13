import type { ContextMenuButton } from "./global"

export type DirectoryInfo = {
    path: string,
    folders: Array<string>,
    files: Array<string>
}

export let folderContextButtons: ContextMenuButton[] = [
    {
        title: "Pin Current Directory",
        callback: () => {
            
        }
    }
];

export let fileContextButtons: ContextMenuButton[] = [
];

export function createFileContextButtons(file: string): ContextMenuButton[] {
    return [
        {
            title: "Open File",
            callback: () => {
                console.log("Open File from context menu clicked");
            }
        },
        {
            title: "Open with...",
            callback: () => {
                console.log("Open with clicked");
            }
        },
        {
            title: "Rename",
            callback: () => {
                console.log("Rename clicked");
            }
        }
    ]
}