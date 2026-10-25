//use eframe::egui;

use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::fs;
use std::path::{Path, PathBuf};

mod markdown;
mod user;

// This is perfectly valid at compile time because it does not allocate heap memory
const IGNORED_DIRS: &[&str] = &[
    "Templates",
    "Presentations",
    "References",
    "Daily Notes",
    "GAMM",
    "Files",
    "PhD Hierarchy",
    "Questions and Ideas",
];

#[derive(Debug)]
pub struct ValidVaultDir {
    path: PathBuf,
}

impl ValidVaultDir {
    /// Creates a new `ValidVaultDir`
    ///
    /// # Errors
    ///
    /// Will return a `String` error variant if
    /// * the provided path string is empty
    /// * the path does not exist
    /// * the path is a file instead of directory
    /// * the directory is a valid .obsidan file/config
    pub fn new(path: String) -> Result<Self, String> {
        if path.trim().is_empty() {
            return Err("Path cannot be empty".to_string());
        }

        let path = PathBuf::from(path);
        let Some(metadata) = fs::metadata(&path).ok() else {
            return Err(format!("Path does not exist: {}", path.display()));
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

        Ok(Self { path })
    }
}

impl AsRef<Path> for ValidVaultDir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

// 2. THE APPLICATION LOGIC
fn main() {
    let input = String::from(concat!(
        // r"G:\.shortcut-targets-by-id\",
        // r"1-hr1TtoeKYtwFu9JoidN2le9Fd98y014\Obsidian\",
        // r"My_Obsidian_Notes1",
        r"/home/p70088775/google_drive",
        r"/Obsidian/My_Obsidian_Notes1/"
    ));
    let vault_dir = ValidVaultDir::new(input);

    match vault_dir {
        Ok(valid_vault) => {
            println!("Valid Obsidian vault path: {}", valid_vault.path.display());
            // let all_directories = list_all_directories(&valid_vault).unwrap_or_else(|| Vec::new());
            let all_directories =
                list_all_directories(valid_vault.as_ref()).unwrap_or_default();
            let all_directories =
                filter_ignored_dirs(all_directories, IGNORED_DIRS).unwrap_or_default();

            let (maybe_list_of_markdown_files, maybe_map_to_markdown_file_location) =
                index_markdown_files(all_directories);
            let list_of_markdown_files = maybe_list_of_markdown_files.unwrap_or_default();
            let _map_to_markdown_file_locations =
                maybe_map_to_markdown_file_location.unwrap_or_default();

            println!("List of markdown files:");
            for file in &list_of_markdown_files {
                println!(
                    " - {}",
                    file.file_stem().unwrap_or_default().to_string_lossy()
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

fn list_all_directories(path: &Path) -> Option<Vec<PathBuf>> {
    let entries = fs::read_dir(path).ok()?;
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            dirs.push(entry_path);
        }
    }
    if dirs.is_empty() { None } else { Some(dirs) }
}

fn filter_ignored_dirs(
    dir_list: Vec<PathBuf>,
    ignored_dirs: &[&str],
) -> Option<Vec<PathBuf>> {
    let ignored_set: HashSet<&str> = ignored_dirs.iter().copied().collect();

    let remaining_dirs: Vec<PathBuf> = dir_list
        .into_iter()
        .filter(|dir| {
            dir.file_name()
                .and_then(|n| n.to_str())
                .is_none_or(|name| !ignored_set.contains(name))
        })
        .collect();

    if remaining_dirs.is_empty() {
        None
    } else {
        Some(remaining_dirs)
    }
}

fn insert_with_disambiguation(path: PathBuf, map: &mut HashMap<String, PathBuf>) {
    // Elegant extraction with an early-return else block
    let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) else {
        return; // Skip files without valid stems
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
    fn process_entries(entries: Vec<PathBuf>, map: &mut HashMap<String, PathBuf>) {
        for entry in entries {
            if entry.is_dir() {
                // Read the directory and recursively process its children
                if let Ok(reader) = std::fs::read_dir(&entry) {
                    let sub_entries: Vec<PathBuf> = reader
                        .filter_map(std::result::Result::ok)
                        .map(|dir_entry| dir_entry.path())
                        .collect();
                    process_entries(sub_entries, map);
                }
            } else if entry.is_file() {
                // Check if the file extension is markdown
                if let Some(ext) = entry.extension()
                    && (ext == "md" || ext == "markdown")
                {
                    insert_with_disambiguation(entry, map);
                }
            }
        }
    }
    let mut file_to_location_map: HashMap<String, PathBuf> = HashMap::new();

    process_entries(entries, &mut file_to_location_map);
    // 1. Check emptiness once up front
    let is_empty = file_to_location_map.is_empty();

    // 2. Simply extract and clone the PathBuf values by borrowing the map via `.values()`
    let final_list: Option<Vec<std::path::PathBuf>> =
        (!is_empty).then(|| file_to_location_map.values().cloned().collect());

    // 3. Wrap the map into an Option without destroying it.
    // .then() with a closure lazily handles the Option wrapping dynamically.
    let final_map: Option<std::collections::HashMap<String, std::path::PathBuf>> =
        (!is_empty).then_some(file_to_location_map);

    (final_list, final_map)
}
