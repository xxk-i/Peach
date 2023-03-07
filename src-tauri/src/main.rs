// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::read_dir;

//static DIRECTORY: &str = "K:/ROMS/Switch/Modding/theatrythm/romfs/StreamingAssets/aa/Switch";

#[tauri::command]
// should exported rust function use ts or rust naming convention lol?
fn listdir(dir: String) -> Vec<String> {
    // iterate the DirEntry's, get file name as string, collect back up
    read_dir(dir).unwrap().map(|x| 
        x.unwrap().file_name().to_str().unwrap().to_string()).collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![listdir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
