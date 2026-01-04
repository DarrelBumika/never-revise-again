use sha1::{Sha1, Digest};
use std::fs;
use std::io::Result;

use crate::constant::path::VCS_OBJECTS_DIR;

pub fn hash_object(data: &[u8]) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn store_object(data: &[u8]) -> Result<String> {
    let hash = hash_object(data);
    let dir = format!("{}/{}", VCS_OBJECTS_DIR, &hash[..2]);
    let path = format!("{}/{}", dir, &hash[2..]);

    fs::create_dir_all(&dir)?;
    fs::write(path, data)?;

    Ok(hash)
}