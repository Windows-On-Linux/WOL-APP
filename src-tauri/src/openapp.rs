use std::process::Command;

pub fn open(path: String) -> bool {
    let command_mod = Command::new("/usr/bin/terminator")
    .arg("-e")
    .arg(format!("bash {:?}; exec bash", path))
    .spawn();
    let command_mod_result = match command_mod {
        Ok(command_mod) => true,
        Err(e) => false
    };
    command_mod_result
}