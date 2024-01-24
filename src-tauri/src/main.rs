// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::{Path, PathBuf};

use tauri::WindowEvent;
use windows::core::HSTRING;
use windows::Win32::Storage::FileSystem::{FindFirstVolumeW, GetDiskFreeSpaceExW, GetLogicalDriveStringsW, GetLogicalDrives};

#[derive(serde::Serialize)]
struct MountedVolume {
    name: Option<String>,
    letter: char,
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
    let mut volume_name: [u16; 1000] = [0; 1000];

    unsafe {
        // The FindFirstVolumeW wrapper from the windows crate automatically calls
        // GetLastError and combines everything into its return Result
        // so we don't have to do that
        let handle = FindFirstVolumeW(&mut volume_name).unwrap();
        println!("{}", String::from_utf16_lossy(&volume_name))
    }
}

#[tauri::command]
async fn get_drives_string() {
    let mut buffer: [u16; 1000] = [0; 1000];

    unsafe {
        let return_value = GetLogicalDriveStringsW(Some(&mut buffer));
        if return_value > 1000 {
            println!("Buffer was not big enough for get_drives()");
        } else if return_value == 0{
            println!("get_drives() failed (return_value is 0)");
        } else {
            println!("{}", String::from_utf16_lossy(&buffer));
        }
    }
}

#[tauri::command]
async fn get_drives() -> Vec<MountedVolume> {
    let mut mounted_volumes: Vec<MountedVolume> = Vec::new();
    let mut drives: u32;
    let letters = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

    unsafe {
        drives = GetLogicalDrives();
    }

    if drives == 0 {
        println!("get_drives() failed (drives is 0)");
    } else {
        // drive represents a bitmask of each available disk, mapped to a letter (e.g. C:)
        for i in 0..letters.len() {
            let drive = (drives >> i) & 1;
            if drive == 1 {
                mounted_volumes.push(
                    get_disk_space_info(letters[i])
                );
            }
        }
    }

    // let mut output = String::new();
    // for i in 0..available_drives.len() - 2 {
    //     output.push_str(format!("{}:\\, ", available_drives.chars().nth(i).unwrap()).as_str());
    // }
    // output.push_str(format!("{}:\\", available_drives.chars().nth(available_drives.len() - 1).unwrap()).as_str());

    // println!("{output}")

    mounted_volumes
}

// https://stackoverflow.com/questions/74173128/how-to-get-a-pcwstr-object-from-a-path-or-string
fn get_disk_space_info(disk: char) -> MountedVolume {
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

fn main() {
  tauri::Builder::default()
    // Hack from https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495 that makes resizing really fast
    .on_window_event(|e| {
        if let WindowEvent::Resized(_) = e.event() {
            std::thread::sleep(std::time::Duration::from_nanos(1));
        }
    })
    .invoke_handler(tauri::generate_handler![get_drives, get_volumes, get_files_at_path])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
