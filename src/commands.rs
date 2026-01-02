use crate::repository;

pub fn init() {
    if let Err(e) = repository::init_repository() {
        eprintln!("Failed to initialize repository: {}", e);
    }
}