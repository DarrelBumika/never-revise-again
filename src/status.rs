use std::io::{BufRead, BufReader, Result};
use std::fs::{read_dir, File, DirEntry};
use std::path::{Path, PathBuf};

use colored::Colorize;

use crate::constant::system::DEFAULT_IGNORED_PATHS;
use crate::constant::path::VCS_INDEX_FILE;

pub fn get_status() -> Result<()> {
    let index = &get_indexed_entries();
    let ignored_by_default = &DEFAULT_IGNORED_PATHS
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let ignored_by_user = &Vec::new(); // TODO Use this from .nraignore

    if !index.is_empty() {
        println!("Ready to save:");
        println!("(use \"nra save <message>\" to save the changes)");
        println!("(use \"nra remove <file>...\" to remove from ready to save)");
        index.iter()
            .for_each(|entry| {
                println!("    {}", entry.green())
            });
    }

    let changed_entries = get_changed_entries(
        Path::new("."),
        index,
        ignored_by_default,
        ignored_by_user,
    );

    if !changed_entries.is_empty() {
        println!("Changed files:");
        println!("(use \"nra add <file>...\" to make it ready to save)");

        changed_entries
            .iter()
            .for_each(|entry| {
                println!("    {}", entry.red());
            });
    } else {
        println!("No changed files");
    }

    Ok(())
}

fn get_indexed_entries() -> Vec<String> {
    let Ok(index) = File::open(VCS_INDEX_FILE) else {
        return Vec::new();
    };

    let reader = BufReader::new(index);
    reader
        .lines()
        .filter_map(|line: Result<String>| line.ok())
        .collect()
}

fn is_contain(path: &PathBuf, collection: &Vec<String>) -> bool {
    let path_str = path.to_string_lossy();
    let path_str = path_str.as_ref();

    !collection.iter().any(|i| i == path_str)
}

fn get_changed_entries(
    path: &Path,
    index: &Vec<String>,
    ignored_by_default: &Vec<String>,
    ignored_by_user: &Vec<String>
) -> Vec<String> {
    let Ok(entries) = read_dir(path) else {
        return Vec::new();
    };

    entries
        // Ignore entries with I/O errors and convert Result<DirEntry> to Option<DirEntry>
        .filter_map(|entry: Result<DirEntry>| entry.ok())
        // Map DirEntry to PathBuf
        .map(|entry: DirEntry| entry.path())
        // Filter index
        .filter(|path: &PathBuf| {
            is_contain(path, index)
        })
        // Filter ignored by default
        .filter(|path: &PathBuf| {
            is_contain(path, ignored_by_default)
        })
        // Filter ignored by user
        .filter(|path: &PathBuf| {
            is_contain(path, ignored_by_user)
        })
        // Collect entries and recurse into directories
        .flat_map(|entry_path: PathBuf| {
            if entry_path.is_dir() {
                get_changed_entries(
                    &entry_path,
                    &index,
                    &ignored_by_default,
                    &ignored_by_user
                )
            } else {
                vec![entry_path.to_string_lossy().to_string()]
            }
        })
        .collect()
}