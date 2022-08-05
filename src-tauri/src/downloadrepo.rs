use git2::Repository;

pub async fn DownloadScript(url: String, path: String, name: String) -> bool{
    let dir = dirs::home_dir().unwrap().as_path().join("wol").join("Downloads").join(name);
    let repo = match Repository::clone(url.as_str(), dir) {
        Ok(repo) => true,
        Err(e) => false,
    };
    repo
}