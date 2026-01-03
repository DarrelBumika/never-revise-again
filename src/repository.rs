use std::fs;
use std::path::Path;

use crate::constant::message::{ INIT_SUCCESS, INIT_SUCCESS_EXIST };
use crate::constant::path::{ VCS_DIR, VCS_OBJECTS_DIR, VCS_REFS_CURRENTS_DIR, VCS_CURRENT_FILE };


pub fn init_repository() -> std::io::Result<()> {

    let message: &str = if is_initialized() {
        INIT_SUCCESS_EXIST
    } else {
        INIT_SUCCESS
    };

    fs::create_dir_all(VCS_OBJECTS_DIR)?;
    fs::create_dir_all(VCS_REFS_CURRENTS_DIR)?;
    fs::write(VCS_CURRENT_FILE, "ref: refs/heads/main\n")?;

    println!("{}", message);
    Ok(())
}

pub fn is_initialized() -> bool {
    Path::new(VCS_DIR).exists()
}