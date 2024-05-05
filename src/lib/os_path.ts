import { path } from "@tauri-apps/api";
import { platform } from "@tauri-apps/plugin-os";

async function get_root(path: string): Promise<string> {
    if (await platform() == "windows") {
        return path.slice(0, 3);
    } else {
        return path.slice(0, 1);
    }
}

async function is_root(path: string): Promise<boolean> {
    if (await platform() == "windows") {
        return path.length == 3;
    } else {
        return path === "/";
    }
}

async function push(p: string, folder: string): Promise<string> {
    return path.join(p, folder);
}

async function pop(p: string): Promise<string> {
    return path.join(p, "..");
}

function get_name(p: string): string {
    if (p === "/") {
        return "Root";
    }
    return p.slice(p.lastIndexOf("/") + 1, p.length);
}

export { get_root, is_root, push, pop, get_name };