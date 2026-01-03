use std::fs;

use crate::{object, repository};

pub fn init() {
    if let Err(e) = repository::init_repository() {
        eprintln!("Failed to initialize repository: {}", e);
    }
}

pub fn add(files: &[String]) {
    for file in files {
        if let Ok(content) = fs::read(file) {
            match object::store_object(&content) {
                Ok(hash) => println!("Added {} -> {}", file, hash),
                Err(e) => eprintln!("Failed to store {}: {}", file, e),
            }
        }
    }
}