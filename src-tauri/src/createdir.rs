use std::path::PathBuf;
use std::fs;

pub fn CreateDir(dir: PathBuf) {
    if(!dir.exists()){
        let mut dir1 = fs::create_dir(dir.clone());
        let mut dir2 = fs::create_dir(dir.join("Downloads"));
        match dir1 {
            Ok(_) => dir1,
            Err(_e) => panic!("Error creating directory ~/wol , please check the permissions for application")
        };
        match dir2 {
            Ok(_) => dir2,
            Err(_e) => panic!("Error creating directory ~/wol/Downloads, please check the permissions for application")
        };
    }
}