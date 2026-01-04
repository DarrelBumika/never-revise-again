use const_format::formatcp;

pub const VCS_DIR: &str = ".nra";

pub const VCS_OBJECTS_DIR: &str = formatcp!("{}/objects", VCS_DIR);

pub const VCS_REFS_DIR: &str = formatcp!("{}/refs", VCS_DIR);
pub const VCS_REFS_CURRENTS_DIR: &str = formatcp!("{}/currents", VCS_REFS_DIR);

pub const VCS_CURRENT_FILE: &str = formatcp!("{}/CURRENT", VCS_DIR);

pub const VCS_INDEX_FILE: &str = formatcp!("{}/index", VCS_DIR);