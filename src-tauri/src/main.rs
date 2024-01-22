// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WindowEvent;
use windows::Win32::Storage::FileSystem::{FindFirstVolumeW, FindNextVolumeW, FindVolumeClose, GetLogicalDriveStringsW, GetLogicalDrives};
use windows::Win32::Foundation::{GetLastError, INVALID_HANDLE_VALUE};

#[derive(serde::Serialize)]
struct MountedVolume {
    name: Option<String>,
    letter: String,
    storage_used: u64,
    capacity: u64
}

#[tauri::command]
// async fn get_volumes() -> Vec<MountedVolume> {
async fn get_volumes() {
    let mut volume_name: [u16; 1000] = [0; 1000];

    unsafe {
        // The FindFirstVolumeW wrapper from the windows crate automatically calls
        // GetLastError and combines everything into its return Result
        // so we don't have to do that
        let handle = FindFirstVolumeW(&mut volume_name).unwrap();
        println!("{}", String::from_utf16_lossy(&volume_name))
    }

    // Vec::new()
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
async fn get_drives() {
    let mut drives = 0u32;
    let letters = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut available_drives = String::new();

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
                available_drives.push(letters[i]);
            }
        }
    }

    let mut output = String::new();
    for i in 0..available_drives.len() - 2 {
        output.push_str(format!("{}:\\, ", available_drives.chars().nth(i).unwrap()).as_str());
    }
    output.push_str(format!("{}:\\", available_drives.chars().nth(available_drives.len() - 1).unwrap()).as_str());

    println!("{output}")
}

fn main() {
  tauri::Builder::default()
    // Hack from https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495 that makes resizing really fast
    .on_window_event(|e| {
        if let WindowEvent::Resized(_) = e.event() {
            std::thread::sleep(std::time::Duration::from_nanos(1));
        }
    })
    .invoke_handler(tauri::generate_handler![get_drives, get_volumes])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
