// Dependencies
use std::fs;

// CONSTANTS
const IGNORED_TAGS: &[&str] = &["Person", "noscript", "iframe"];
const IGNORED_FILE_DIRS: &[&str] = &[
    "Canvasses",
    "Daily Notes",
    "Templates",
    "References",
    "Presentations",
    "Files",
    "PhD Hierarchy",
];

fn _read_file(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}
