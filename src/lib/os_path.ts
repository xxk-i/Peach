import { os, path} from "@tauri-apps/api";

async function get_root(path: string): Promise<string> {
    if (await os.type() == "Windows_NT") {
        return path.slice(0, 3);
    } else {
        return path.slice(0, 1);
    }
}

async function is_root(path: string): Promise<boolean> {
    if (await os.type() == "Windows_NT") {
        return path.length == 3;
    } else {
        return path === "/";
    }
}

async function pop(p: string): Promise<string> {
    return path.join(p, "..");
}

export { get_root, is_root, pop };