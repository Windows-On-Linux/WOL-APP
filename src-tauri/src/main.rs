#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde_json::{Value};
extern crate dirs;
mod createdir;
mod readapps;
mod downloadrepo;
mod updaterepo;
mod executescript;
mod openapp;
fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![initapp, read_dir, download_script, update_repo, execute_script, open_app])  
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}


#[tauri::command]
fn initapp() {
  let dir = dirs::home_dir().unwrap().as_path().join("wol");
  createdir::CreateDir(dir.clone());
  readapps::Readapps(dir);
}

#[tauri::command]
fn read_dir() -> Vec<readapps::App>{
  let dir = dirs::home_dir().unwrap().as_path().join("wol");
  readapps::Readapps(dir)
}

#[tauri::command]
async fn download_script(url: String, path: String, name: String) -> bool {
  downloadrepo::DownloadScript(url, path, name).await
}

#[tauri::command]
async fn update_repo(url: String) -> Value {
  updaterepo::UpdateRepo(url).await
}

#[tauri::command]
fn execute_script(name: String) -> bool {
  executescript::execute_script(name)
}

#[tauri::command]
fn open_app(path: String) -> bool {
  openapp::open(path)
}