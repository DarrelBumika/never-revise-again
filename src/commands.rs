use crate::repository;
use crate::repository::is_initialized;
use crate::status;

pub fn init() {
    if let Err(e) = repository::init_repository() {
        eprintln!("Failed to initialize repository: {}", e);
    }
}

pub fn status() {
    if let Ok(()) = check_nra_repository() {
        if let Err(e) = status::get_status() {
            eprintln!("Failed to get status: {}", e);
        }
    }
}

pub fn check_nra_repository() -> Result<(), ()> {
    if !is_initialized() {
        println!("Not a NRA repository");
        println!("(use \"nra init\" to initialize NRA repository)");

        return Err(())
    }

    Ok(())
}