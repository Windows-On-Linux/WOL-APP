use std::path::{PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct App {
  name: String,
  path: PathBuf,
  startup: PathBuf
}

pub fn Readapps(dir: PathBuf) -> Vec<App>{
    let paths = fs::read_dir(dir).unwrap();
    let mut files = Vec::new();
    for path in paths {
        let file = path.unwrap();
        if(file.metadata().unwrap().is_dir()){
          if(file.path().join("name.txt").exists()){
            let app = App{
              name: format!("{:?}", file.path().as_os_str()).split("/").last().unwrap().to_string(),
              path: file.path().join("name.txt"),
              startup: file.path().join("startup.sh")
            };
            files.push(app);
          }
        };
    }
    files
}