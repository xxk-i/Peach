// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use sysinfo::Disks;

use tauri::WindowEvent;

#[derive(serde::Serialize)]
struct MountedVolume {
    name: Option<String>,
    mount_point: String,
    storage_used: u64,
    capacity: u64
}

#[derive(serde::Serialize)]
struct DirectoryInfo {
    folders: Vec<String>,
    files: Vec<String>,
}

#[tauri::command]
async fn get_volumes() {
    /*
    let mut volume_name: [u16; 1000] = [0; 1000];

    unsafe {
        // The FindFirstVolumeW wrapper from the windows crate automatically calls
        // GetLastError and combines everything into its return Result
        // so we don't have to do that
        let handle = FindFirstVolumeW(&mut volume_name).unwrap();
        println!("{}", String::from_utf16_lossy(&volume_name))
    }
    */
}

#[tauri::command]
async fn get_drives_string() {
    // let mut buffer: [u16; 1000] = [0; 1000];

    // unsafe {
    //     let return_value = GetLogicalDriveStringsW(Some(&mut buffer));
    //     if return_value > 1000 {
    //         println!("Buffer was not big enough for get_drives()");
    //     } else if return_value == 0{
    //         println!("get_drives() failed (return_value is 0)");
    //     } else {
    //         println!("{}", String::from_utf16_lossy(&buffer));
    //     }
    // }
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

    let paths = fs::read_dir(PathBuf::from(path)).unwrap();

    for entry in paths {
        let path_type = entry.as_ref().unwrap().file_type().unwrap();
        if path_type.is_dir() {
            folders.push(entry.unwrap().file_name().to_str().unwrap().to_owned());
        } else if path_type.is_file() {
            files.push(entry.unwrap().file_name().to_str().unwrap().to_owned());
        }
    }

    DirectoryInfo {
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
        // Hack from https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495 that makes resizing really fast
        .on_window_event(|e| {
            if let WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .invoke_handler(tauri::generate_handler![get_drives, get_volumes, get_files_at_path, open_file, get_disk_space_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
