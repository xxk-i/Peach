// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use sysinfo::Disks;

use tauri::WindowEvent;

mod icon_cache;
use icon_cache::IconCache;
use icon_cache::InstalledApplication;

#[derive(serde::Serialize)]
struct MountedVolume {
    name: Option<String>,
    mount_point: String,
    storage_used: u64,
    capacity: u64
}

#[derive(serde::Serialize)]
struct DirectoryInfo {
    path: String,
    folders: Vec<String>,
    files: Vec<String>,
}

struct AppState {
    icon_cache: Mutex<Option<IconCache>>
}

#[tauri::command]
async fn get_applications(app_handle: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<Vec<InstalledApplication>, String> {
    let mut icon_cache = state.icon_cache.lock().unwrap();
    if cfg!(target_os = "macos") {
        if icon_cache.is_none() {
            *icon_cache = Some(IconCache::initialize(app_handle.path_resolver().app_cache_dir().unwrap()));
        }
        return Ok(icon_cache.as_ref().unwrap().apps.clone())
    }

    Err(String::from("Unsupported"))
    // Ok(vec![InstalledApplication {
    //     name: String::from("hi"),
    //     icon: String::from("hi"),
    //     path: String::from("hi"),
    // }])
}

// https://stackoverflow.com/questions/74173128/how-to-get-a-pcwstr-object-from-a-path-or-string
#[tauri::command]
async fn get_disk_space_info(disk: char) -> MountedVolume {
    /*
    let mut root_path = String::from(disk);
    root_path.push_str(":\\");

    let mut free_bytes_available: u64 = 0;
    let mut total_bytes: u64 = 0;
    let mut free_bytes: u64 = 0;

    unsafe {
        GetDiskFreeSpaceExW(&HSTRING::from(root_path), Some(&mut free_bytes_available), Some(&mut total_bytes), Some(&mut free_bytes)).expect("Failed to get bytes for disk");
    }

    let total_gigabytes: u64 = total_bytes / 1024 / 1024 / 1024;

    MountedVolume {
        name: Some(String::from("Disk")),
        letter: disk,
        storage_used: (total_bytes - free_bytes) / 1024 / 1024 / 1024,
        capacity: total_gigabytes,
    }
    */
    MountedVolume {
        name: Some(String::from("Disk")),
        mount_point: String::from("/"),
        storage_used: 3,
        capacity: 64
    }
}

#[tauri::command]
async fn get_drives() -> Vec<MountedVolume> {
    let disks = Disks::new_with_refreshed_list();
    let mut mounted_volumes = Vec::new();
    for disk in &disks {
        mounted_volumes.push(
            MountedVolume {
                name: Some(disk.name().to_str().unwrap().to_owned()),
                mount_point: disk.mount_point().to_str().unwrap().to_owned(),
                storage_used: disk.total_space() - disk.available_space(),
                capacity: disk.total_space()
            }
        );
    }

    mounted_volumes
}

#[tauri::command]
async fn get_files_at_path(path: String) -> DirectoryInfo {
    let mut folders = Vec::new();
    let mut files = Vec::new();

    let paths = fs::read_dir(PathBuf::from(&path)).unwrap();

    for entry in paths {
        let path_type = entry.as_ref().unwrap().file_type().unwrap();
        if path_type.is_dir() {
            folders.push(entry.unwrap().file_name().to_str().unwrap().to_owned());
        } else if path_type.is_file() {
            files.push(entry.unwrap().file_name().to_str().unwrap().to_owned());
        }
    }

    DirectoryInfo {
        path,
        folders,
        files
    } 
}

#[tauri::command]
async fn open_file(path: String) {
    open::that(path).expect("Failed to open file");
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { icon_cache: Mutex::new(None) })
        // Hack from https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495 that makes resizing really fast
        .on_window_event(|e| {
            if let WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .invoke_handler(tauri::generate_handler![get_drives, get_files_at_path, open_file, get_disk_space_info, get_applications])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
