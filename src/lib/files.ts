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
    {
        title: "Open File",
        callback: () => {
            console.log("Open File Clicked");
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
];