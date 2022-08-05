use std::{process::Command, fmt::format};

pub fn execute_script(name: String) -> bool{
    let dir = dirs::home_dir().unwrap().as_path().join("wol").join("Downloads").join(name).join("build.sh");
    let command_mod = Command::new("chmod")
    .arg("+x")
    .arg(dir.as_os_str())
    .spawn();
    let command_mod_result = match command_mod {
        Ok(command_mod) => true,
        Err(e) => false
    };
    if(command_mod_result){
        let command = Command::new("/usr/bin/terminator")
            .arg("-e")
            .arg(format!("bash {:?}; exec bash", dir.as_os_str()))
            .spawn();
        let result = match command{
            Ok(mut child) => true,
            Err(err) => false,
        };
        result
    }else{
        false
    }

}