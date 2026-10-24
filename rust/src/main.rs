//use eframe::egui;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

mod markdown;
mod user;
//use crate::user::options::{AppSettings, AppearanceOptions};
//
//struct MyApp {
//    path_input: String,
//    settings: AppSettings,
//}
//
//impl Default for MyApp {
//    fn default() -> Self {
//        Self {
//            path_input: String::new(),
//            settings: AppSettings {
//                appearance: AppearanceOptions::System,
//                vault_path: None,
//            },
//        }
//    }
//}

#[derive(Debug)]
pub struct ValidVaultDir {
    path: PathBuf,
}

impl ValidVaultDir {
    pub fn new(path: String) -> Result<Self, String> {
        if path.trim().is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        // we need to check if the path exists
        // it is a directory
        // it is a valid Obsidian vault (contains a .obsidian folder)

        let path = PathBuf::from(path);
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => {
                return Err(format!("Path does not exist: {}", path.display()));
            }
        };

        if !metadata.is_dir() {
            return Err(format!("Path is not a directory: {}", path.display()));
        }
        let obsidian_dir = path.join(".obsidian");
        let obsidian_metadata = fs::metadata(&obsidian_dir).map_err(|_| {
            format!(
                "Path is not a valid Obsidian vault (missing .obsidian folder): {}",
                obsidian_dir.display()
            )
        })?;

        if !obsidian_metadata.is_dir() {
            return Err(format!(
                "Path is not a valid Obsidian vault (missing .obsidian folder): {}",
                obsidian_dir.display()
            ));
        }

        Ok(ValidVaultDir { path })
    }
}

// 2. THE APPLICATION LOGIC
fn main() {
    let input = String::from(concat!(
        r"G:\.shortcut-targets-by-id\",
        r"1-hr1TtoeKYtwFu9JoidN2le9Fd98y014\Obsidian\",
        r"My_Obsidian_Notes1",
    ));
    let vault_dir = ValidVaultDir::new(input);

    match vault_dir {
        Ok(valid_vault) => {
            println!("Valid Obsidian vault path: {}", valid_vault.path.display());
            // Here you can proceed with your application logic using the valid vault path
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            // Handle the error, e.g., prompt the user for a valid path
            return;
        }
    }
}

fn list_all_directories(path: &Path) -> Option<Vec<PathBuf>> {
    let entries = fs::read_dir(path).ok()?;
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                dirs.push(entry_path);
            }
        }
    }
    if dirs.is_empty() { None } else { Some(dirs) }
}

fn filter_ignored_dirs(
    dir_list: Vec<PathBuf>,
    ignored_dirs: &[PathBuf],
) -> Option<Vec<PathBuf>> {
    let ignored_set: HashSet<&PathBuf> = ignored_dirs.iter().collect();

    let remaining_dirs: Vec<PathBuf> = dir_list
        .into_iter()
        .filter(|dir| !ignored_set.iter().any(|ignored| dir.starts_with(ignored)))
        .collect();

    if remaining_dirs.is_empty() {
        None
    } else {
        Some(remaining_dirs)
    }
}
fn insert_with_disambiguation(path: PathBuf, map: &mut HashMap<String, PathBuf>) {
    let file_stem = match path.file_stem().and_then(|s| s.to_str()) {
        Some(stem) => stem,
        None => return, // Skip files without valid stems
    };

    let mut modifier_components: Vec<String> = Vec::new();
    let mut current_ancestor = path.parent();

    loop {
        // Construct the key candidate: "file_stem" + "_parent1" + "_parent2" etc.
        let mut key = file_stem.to_string();
        for component in modifier_components.iter().rev() {
            key.push('_');
            key.push_str(component);
        }

        // If the key doesn't exist, or if it points to the exact same path, we can safely insert it
        if !map.contains_key(&key) || map.get(&key) == Some(&path) {
            map.insert(key, path);
            break;
        }

        // If key exists, grab the next parent directory to differentiate it
        if let Some(parent) = current_ancestor {
            if let Some(parent_name) = parent.file_name().and_then(|n| n.to_str()) {
                modifier_components.push(parent_name.to_string());
                current_ancestor = parent.parent();
            } else {
                // No more readable parent directory names left; append a fallback to prevent infinite loop
                modifier_components.push("duplicate".to_string());
            }
        } else {
            modifier_components.push("duplicate".to_string());
        }
    }
}

fn index_markdown_files(
    entries: Vec<PathBuf>,
) -> (Option<Vec<PathBuf>>, Option<HashMap<String, PathBuf>>) {
    let mut list_of_markdown_files: Vec<PathBuf> = Vec::new();
    let mut file_to_location_map: HashMap<String, PathBuf> = HashMap::new();

    fn process_entries(
        entries: Vec<PathBuf>,
        list: &mut Vec<PathBuf>,
        map: &mut HashMap<String, PathBuf>,
    ) {
        for entry in entries {
            if entry.is_dir() {
                // Read the directory and recursively process its children
                if let Ok(reader) = std::fs::read_dir(&entry) {
                    let sub_entries: Vec<PathBuf> = reader
                        .filter_map(|res| res.ok())
                        .map(|dir_entry| dir_entry.path())
                        .collect();
                    process_entries(sub_entries, list, map);
                }
            } else if entry.is_file() {
                // Check if the file extension is markdown
                if let Some(ext) = entry.extension() {
                    if ext == "md" || ext == "markdown" {
                        list.push(entry.clone());
                        insert_with_disambiguation(entry, map);
                    }
                }
            }
        }
    }
    process_entries(
        entries,
        &mut list_of_markdown_files,
        &mut file_to_location_map,
    );
    let final_list = if list_of_markdown_files.is_empty() {
        None
    } else {
        Some(list_of_markdown_files)
    };
    let final_map = if file_to_location_map.is_empty() {
        None
    } else {
        Some(file_to_location_map)
    };
    (final_list, final_map)
}

// fn index_markdown_files_with_update(
//     entries: Vec<PathBuf>,
// ) -> (Option<HashSet<PathBuf>>, Option<HashMap<String, PathBuf>>) {
//     let mut set_of_markdown_files: HashSet<PathBuf> = HashSet::new();
//     let mut file_to_location_map: HashMap<String, PathBuf> = HashMap::new();
//     fn return_unique_name(entry: PathBuf, set: &HashSet<PathBuf>) {
//         let file_stem = match path.file_stem().and_then(|s| s.to_str()) {
//             Some(stem) => stem,
//             None => return, // Skip files without valid stems
//         };
//
//         let mut modifier_components: Vec<String> = Vec::new();
//         let mut current_ancestor = path.parent();
//
//         loop {
//             // Construct the key candidate: "file_stem" + "_parent1" + "_parent2" etc.
//             let mut key = file_stem.to_string();
//             for component in modifier_components.iter().rev() {
//                 key.push('_');
//                 key.push_str(component);
//             }
//
//             // If the key doesn't exist, or if it points to the exact same path, we can safely insert it
//             if !map.contains_key(&key) || map.get(&key) == Some(&path) {
//                 map.insert(key, path);
//                 break;
//             }
//
//             // If key exists, grab the next parent directory to differentiate it
//             if let Some(parent) = current_ancestor {
//                 if let Some(parent_name) = parent.file_name().and_then(|n| n.to_str()) {
//                     modifier_components.push(parent_name.to_string());
//                     current_ancestor = parent.parent();
//                 } else {
//                     // No more readable parent directory names left; append a fallback to prevent infinite loop
//                     modifier_components.push("duplicate".to_string());
//                 }
//             } else {
//                 modifier_components.push("duplicate".to_string());
//             }
//         }
//     }
//     fn process_entries(
//         entries: Vec<PathBuf>,
//         set: &mut HashSet<PathBuf>,
//         map: &mut HashMap<String, PathBuf>,
//     ) {
//         for entry in entries {
//             if entry.is_dir() {
//                 // Read the directory and recursively process its children
//                 if let Ok(reader) = std::fs::read_dir(&entry) {
//                     let sub_entries: Vec<PathBuf> = reader
//                         .filter_map(|res| res.ok())
//                         .map(|dir_entry| dir_entry.path())
//                         .collect();
//                     process_entries(sub_entries, set, map);
//                 }
//             } else if entry.is_file() {
//                 // Check if the file extension is markdown
//                 if let Some(ext) = entry.extension() {
//                     if ext == "md" || ext == "markdown" {
//                         insert_with_disambiguation(entry, map);
//                     }
//                 }
//             }
//         }
//     }
//     process_entries(
//         entries,
//         &mut list_of_markdown_files,
//         &mut file_to_location_map,
//     );
//     let final_list = if list_of_markdown_files.is_empty() {
//         None
//     } else {
//         Some(list_of_markdown_files)
//     };
//     let final_map = if file_to_location_map.is_empty() {
//         None
//     } else {
//         Some(file_to_location_map)
//     };
//     (final_list, final_map)
// }
