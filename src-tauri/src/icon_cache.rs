use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use tauri_icns::{IconFamily, IconType};

#[derive(serde::Serialize, Clone)]
pub struct InstalledApplication {
    pub name: String,
    pub icon: String,
    pub path: String,
}

pub struct IconCache {
    pub apps: Vec<InstalledApplication>,
    converted_icons: HashSet<String>,
    icon_cache_dir: PathBuf,
}

impl IconCache {
    pub fn initialize(app_cache_dir: PathBuf) -> IconCache {
        let mut icon_cache_dir = app_cache_dir.clone();
        icon_cache_dir.push("icons");

        if icon_cache_dir.metadata().is_err() {
            std::fs::create_dir_all(&icon_cache_dir).unwrap();
        }

        let mut cache = IconCache {
            apps: Vec::new(),
            converted_icons: HashSet::new(),
            icon_cache_dir,
        };

        cache.get_applications_macos();
        cache.check_existing();
        cache.convert_icons();

        cache
    }

    fn check_existing(&mut self) {
        for entry in fs::read_dir(&self.icon_cache_dir).unwrap() {
            let entry = entry.unwrap();
            self.converted_icons.insert(
                entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .strip_suffix(".png")
                    .unwrap()
                    .to_owned(),
            );
        }
    }

    // Converts leftover icons not contained in self.converted_icons
    fn convert_icons(&mut self) {
        for app in &self.apps {
            if self.converted_icons.contains(&app.name) {
                continue;
            }

            let mut png_path = self.icon_cache_dir.clone();
            png_path.push(format!("{}.png", app.name));
            // Load an icon family from an ICNS file.
            let file = BufReader::new(File::open(&app.icon).unwrap());
            let icon_family = IconFamily::read(file).unwrap();

            // Extract an icon from the family and save it as a PNG.
            // A lot of these don't work on my machine (jpeg2000)
            // so we iterate icontypes (best quality -> worst quality)
            // until something works
            let image = match icon_family.get_icon_with_type(IconType::RGBA32_512x512_2x) {
                Ok(i) => i,
                Err(_) => match icon_family.get_icon_with_type(IconType::RGBA32_512x512) {
                    Ok(i) => i,
                    Err(_) => match icon_family.get_icon_with_type(IconType::RGBA32_256x256_2x) {
                        Ok(i) => i,
                        Err(_) => match icon_family.get_icon_with_type(IconType::RGBA32_256x256) {
                            Ok(i) => i,
                            Err(e) => {
                                println!("Failed to convert: {}, {}", app.name, e);
                                continue;
                            }
                        },
                    },
                },
            };

            let file = BufWriter::new(File::create(png_path).unwrap());
            image.write_png(file).unwrap();

            self.converted_icons.insert(app.name.clone());
        }
    }

    fn get_applications_macos(&mut self) {
        let paths = fs::read_dir(PathBuf::from("/Applications")).unwrap();

        for entry in paths {
            let path_type = entry.as_ref().unwrap().file_type().unwrap();
            if path_type.is_dir()
                && entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .ends_with(".app")
            {
                let icon_path = format!(
                    "/Applications/{}/{}",
                    entry.as_ref().unwrap().file_name().to_str().unwrap(),
                    "Contents/Resources"
                );
                if let Ok(resources) = fs::read_dir(PathBuf::from(icon_path)) {
                    for resource in resources {
                        if resource.as_ref().unwrap().file_type().unwrap().is_file()
                            && resource
                                .as_ref()
                                .unwrap()
                                .file_name()
                                .to_str()
                                .unwrap()
                                .ends_with(".icns")
                        {
                            let name = entry
                                .as_ref()
                                .unwrap()
                                .file_name()
                                .to_str()
                                .unwrap()
                                .strip_suffix(".app")
                                .unwrap()
                                .to_owned();
                            self.apps.push(InstalledApplication {
                                name,
                                icon: resource
                                    .as_ref()
                                    .unwrap()
                                    .path()
                                    .to_str()
                                    .unwrap()
                                    .to_owned(),
                                path: entry.as_ref().unwrap().path().to_str().unwrap().to_owned(),
                            });
                            break;
                        }
                    }
                }
            }
        }
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
