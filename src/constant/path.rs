use const_format::formatcp;

pub const VCS_DIR: &str = ".nra";

pub const VCS_OBJECTS_DIR: &str = formatcp!("{}/objects", VCS_DIR);

pub const VCS_REFS_DIR: &str = formatcp!("{}/refs", VCS_DIR);
pub const VCS_REFS_HEADS_DIR: &str = formatcp!("{}/heads", VCS_REFS_DIR);

pub const VCS_HEAD_FILE: &str = formatcp!("{}/HEAD", VCS_DIR);