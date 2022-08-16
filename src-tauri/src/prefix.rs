use std::process::Command;
use std::env;

pub fn open_app_in_prefix(path: &str, prefix: &str) -> bool{
    env::set_var("WINEPREFIX", prefix);
    let command_mod = Command::new("/usr/bin/terminator")
    .arg("-e")
    .arg(format!("wine {:?}; exec bash", path))
    .spawn();
    let command_mod_result = match command_mod {
        Ok(command_mod) => true,
        Err(e) => false
    };
    command_mod_result
}

pub fn cfg(prefix: &str) -> bool{
    env::set_var("WINEPREFIX", prefix);
    let command_mod = Command::new("/usr/bin/terminator")
    .arg("-e")
    .arg(format!("winecfg; exec bash"))
    .spawn();
    let command_mod_result = match command_mod {
        Ok(command_mod) => true,
        Err(e) => false
    };
    command_mod_result
}