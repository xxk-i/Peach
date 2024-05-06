// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use sysinfo::Disks;

use tauri::async_runtime::channel;
use tauri::async_runtime::spawn;
use tauri::async_runtime::Sender;
use tauri::image::Image;
use tauri::menu::MenuBuilder;
use tauri::menu::MenuEvent;
use tauri::menu::MenuItemBuilder;
use tauri::menu::PredefinedMenuItem;
use tauri::App;
use tauri::AppHandle;
use tauri::Manager;
use tauri::RunEvent;

mod discover;

mod icon_cache;
use icon_cache::IconCache;
use icon_cache::InstalledApplication;

#[derive(serde::Serialize)]
struct MountedVolume {
    name: Option<String>,
    mount_point: String,
    storage_used: u64,
    capacity: u64,
}

#[derive(serde::Serialize)]
struct DirectoryInfo {
    path: String,
    folders: Vec<String>,
    files: Vec<String>,
}

struct AppState<'a> {
    icon_cache: Mutex<Option<IconCache>>,
    discover_tx: Mutex<Option<Sender<&'a str>>>,
}

#[tauri::command]
async fn get_applications(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState<'_>>,
) -> Result<Vec<InstalledApplication>, String> {
    let mut icon_cache = state.icon_cache.lock().unwrap();
    if cfg!(target_os = "macos") {
        if icon_cache.is_none() {
            *icon_cache = Some(IconCache::initialize(
                app_handle.path().app_cache_dir().unwrap(),
            ));
        }
        return Ok(icon_cache.as_ref().unwrap().apps.clone());
    }

    Err(String::from("Unsupported"))
    // Ok(vec![InstalledApplication {
    //     name: String::from("hi"),
    //     icon: String::from("hi"),
    //     path: String::from("hi"),
    // }])
}

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
        capacity: 64,
    }
}

#[tauri::command]
async fn get_drives() -> Vec<MountedVolume> {
    let disks = Disks::new_with_refreshed_list();
    let mut mounted_volumes = Vec::new();
    for disk in &disks {
        mounted_volumes.push(MountedVolume {
            name: Some(disk.name().to_str().unwrap().to_owned()),
            mount_point: disk.mount_point().to_str().unwrap().to_owned(),
            storage_used: disk.total_space() - disk.available_space(),
            capacity: disk.total_space(),
        });
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
        files,
    }
}

#[tauri::command]
async fn open_file(path: String) {
    open::that(path).expect("Failed to open file");
}

#[tauri::command]
async fn open_dev_tools(window: tauri::Window) {
    println!("dev tools opening");
    let webview_window = window.get_webview_window("main").unwrap();
    if webview_window.is_devtools_open() {
        webview_window.close_devtools();
    } else {
        webview_window.open_devtools();
    }
}

fn handle_system_tray_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "end_discovery" => {
            let state: tauri::State<'_, AppState> = app.state();
            if let Some(tx) = state.discover_tx.lock().unwrap().as_mut() {
                match tx.try_send("terminate") {
                    Err(e) => println!("Failed to terminate discovery service: {e}"),
                    _ => {}
                };
            };
        }
        _ => {}
    }
}

fn setup(app: &App) {
    let state: tauri::State<'_, AppState> = app.state();
    let (discover_tx, discover_rx) = channel(1);
    *state.discover_tx.lock().unwrap() = Some(discover_tx);
    spawn(async { discover::start(discover_rx).await });
}

fn main() {
    // Build the application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        // Default app state (empty mutexes)
        .manage(AppState {
            icon_cache: Mutex::new(None),
            discover_tx: Mutex::new(None),
        })
        // Hack from https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495 that makes resizing really fast
        // Seems fixed on Tauri 2.0?
        // .on_window_event(|w, e| {
        //     if let WindowEvent::Resized(_) = e {
        //         std::thread::sleep(std::time::Duration::from_nanos(1));
        //     }
        // })
        // Add our rust functions callable from the frontend
        .invoke_handler(tauri::generate_handler![
            get_drives,
            get_files_at_path,
            open_file,
            get_disk_space_info,
            get_applications,
            open_dev_tools
        ])
        // Add our system tray
        .setup(|app| {
            // System tray menu
            // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
            let end_discovery =
                MenuItemBuilder::with_id("end_discovery", "End Discovery").build(app)?;
            let quit = PredefinedMenuItem::quit(app, "Quit Peach".into())?;

            let menu = MenuBuilder::new(app)
                .items(&[&end_discovery, &quit])
                .build()?;
            let tray = tauri::tray::TrayIconBuilder::with_id("peach-tray")
                .menu(&menu)
                .on_menu_event(handle_system_tray_event)
                // Icon is now set in here instead of tauri.conf.json
                .icon(Image::from_path("icons/PeachTrayIcon.png")?)
                .build(app)?;
            app.get_webview_window("main").unwrap().open_devtools();
            setup(app);
            Ok(())
        })
        // Build based on tauri_conf.json
        .build(tauri::generate_context!())
        .expect("error building tauri application")
        // Prevent exiting when last window is closed so that
        // our file syncing services can keep running in the
        // background
        .run(|_app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => api.prevent_exit(),
            _ => {}
        });
}
