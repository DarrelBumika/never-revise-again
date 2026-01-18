use std::process::Command;

pub fn git_init() -> Result<(), String> {
    let init_output = Command::new("git")
        .arg("init")
        .arg("-b")
        .arg("main")
        .output()
        .map_err(|err| format!("{:?}", err))?;

    if !init_output.status.success() {
        return Err("Something went wrong...".to_string());
    }

    Ok(())
}