use std::fs;
use colored::Colorize;
use crate::{object, repository};
use crate::constant::path::VCS_INDEX_FILE;
use crate::repository::is_initialized;
use crate::status;

pub fn init() {
    if let Err(e) = repository::init_repository() {
        eprintln!("Failed to initialize repository: {}", e);
    }
}

pub fn add(files: &[String]) {
    for file in files {
        if let Ok(content) = fs::read(file) {
            match object::store_object(&content) {
                Ok(hash) => {
                    fs::write(VCS_INDEX_FILE, file).unwrap();
                    println!("Added {} -> {}", file, hash)
                },
                Err(e) => eprintln!("Failed to store {}: {}", file, e),
            }
        } else {
            eprintln!("{} {}", "Failed to read".red(), file.red());
        }
    }
}

pub fn status() {
    if let Err(err) = is_initialized() {
        println!("{}", err);
        println!("(use \"nra init\" to initialize NRA repository)");
    } else {
        if let Err(e) = status::get_status() {
            eprintln!("Failed to get status: {}", e);
        }
    }
}