use std::{fs::{self, File}, io::{BufReader, BufWriter}, path::PathBuf};
use tauri::PathResolver;
use tauri_icns::{IconFamily, IconType};

#[derive(serde::Serialize)]
pub struct InstalledApplication {
    pub name: String,
    pub icon: String,
    pub path: String
}

pub struct IconCache {
    pub apps: Vec<InstalledApplication>
}

impl IconCache {
    pub fn create() -> IconCache {
        IconCache {
            apps: Self::get_applications_macos()
        }
    }

    pub fn convert_icons(&self, app_handle: tauri::AppHandle) {
        let mut icon_cache_dir = app_handle.path_resolver().app_cache_dir().unwrap();
        icon_cache_dir.push("icons");
        if icon_cache_dir.metadata().is_err() {
            std::fs::create_dir_all(&icon_cache_dir).unwrap();
        }

        for app in &self.apps {
            let mut png_path = icon_cache_dir.clone();
            png_path.push(format!("{}.png", app.name));
            // Load an icon family from an ICNS file.
            let file = BufReader::new(File::open(&app.icon).unwrap());
            let icon_family = IconFamily::read(file).unwrap();

            // Extract an icon from the family and save it as a PNG.
            let image = match icon_family.get_icon_with_type(IconType::RGBA32_256x256) {
                Ok(i) => i,
                Err(e) => continue
            };
            let file = BufWriter::new(File::create(png_path).unwrap());
            image.write_png(file).unwrap();
        }
    }

    fn get_applications_macos() -> Vec<InstalledApplication> {
        let mut apps = Vec::new();
        let paths = fs::read_dir(PathBuf::from("/Applications")).unwrap();

        for entry in paths {
            let path_type = entry.as_ref().unwrap().file_type().unwrap();
            if path_type.is_dir() && entry.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".app") {
                let icon_path = format!("/Applications/{}/{}", entry.as_ref().unwrap().file_name().to_str().unwrap(), "Contents/Resources");
                if let Ok(resources) = fs::read_dir(PathBuf::from(icon_path)) {
                    for resource in resources {
                        if resource.as_ref().unwrap().file_type().unwrap().is_file() && resource.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".icns") {
                            let name = entry.as_ref().unwrap().file_name().to_str().unwrap().strip_suffix(".app").unwrap().to_owned();
                            apps.push(InstalledApplication {
                                name,
                                icon: resource.as_ref().unwrap().path().to_str().unwrap().to_owned(),
                                path: entry.as_ref().unwrap().path().to_str().unwrap().to_owned(),
                            });
                            break;
                        }
                    }
                }
            }
        }
        
        apps
    }
}
// #[cfg(target_os = "macos")]
// fn init_macos() -> IconCache {
//     IconCache {}
// }

// pub fn initialize() -> IconCache {
//     if cfg!(target_os = "macos") {
//         return init_macos();
//     }

//     IconCache {}
// }