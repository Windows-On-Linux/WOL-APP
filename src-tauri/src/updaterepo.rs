use serde_json::{Value};

pub async fn UpdateRepo(url: String) -> Value {
    let resp = reqwest::get(url)
    .await;
    let data = resp.unwrap().text().await.unwrap();
    let parsed:Value = serde_json::from_str(data.as_str()).unwrap();
    parsed
}