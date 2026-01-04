use std::io::{BufRead, BufReader, Result};
use std::fs::{read_dir, File, DirEntry};
use std::path::{Path, PathBuf};

use colored::Colorize;

use crate::constant::system::DEFAULT_IGNORED_PATHS;
use crate::constant::path::VCS_INDEX_FILE;

pub fn get_status() -> Result<()> {
    let index = get_indexed_entries();
    let ignored_by_default = DEFAULT_IGNORED_PATHS;
    let ignored_by_user = Vec::new();

    if !index.is_empty() {
        println!("Ready to save:");
        println!("(use \"nra save <message>\" to save the changes)");
        println!("(use \"nra remove <file>...\" to remove from ready to save)");
        index.iter()
            .for_each(|entry| {
                println!("    {}", entry.green())
            });
    }

    println!("Changed files:");
    println!("(use \"nra add <file>...\" to make it ready to save)");
    get_entries(
        Path::new("."),
        index,
        ignored_by_default,
        ignored_by_user,
    ).iter()
        .for_each(|entry| {
            println!("    {}", entry.red());
        });

    Ok(())
}

fn get_indexed_entries() -> Vec<String> {
    let Ok(index) = File::open(VCS_INDEX_FILE) else {
        return Vec::new();
    };

    let reader = BufReader::new(index);
    reader
        .lines()
        .map(|line: Result<String>| line.unwrap())
        .collect()
}

fn get_entries(
    path: &Path,
    index: Vec<String>,
    ignored_by_default: &[&str],
    ignored_by_user: Vec<String>
) -> Vec<String> {
    let Ok(entries) = read_dir(path) else {
        return Vec::new();
    };

    entries
        // Filter entry by status
        .filter_map(|entry: Result<DirEntry>| entry.ok())
        // Mapping entry to DirEntry
        .map(|entry: DirEntry| entry.path())
        // Filter index
        .filter(|path: &PathBuf|
            !index.contains(
                &path
                    .to_string_lossy()
                    .to_string()
            )
        )
        // Filter ignored by default
        .filter(|path: &PathBuf| {
            !ignored_by_default.contains(
                &path
                    .to_string_lossy()
                    .as_ref()
            )
        })
        // Filter ignored by user
        .filter(|path: &PathBuf| {
            !ignored_by_user.contains(
                &path
                    .to_string_lossy()
                    .to_string()
            )
        })
        // Collect entries and recurse into directories
        .flat_map(|entry_path: PathBuf| {
            let mut entries = vec![entry_path.to_string_lossy().to_string()];
            if entry_path.is_dir() {
                entries.append(&mut get_entries(
                    &entry_path,
                    index.clone(),
                    ignored_by_default,
                    ignored_by_user.clone()
                ));
            }
            entries
        })
        .collect()
}