use reqwest;
use serde::Deserialize;

const REGISTRY_API: &str = "https://api.github.com/repos/healdep/shim-registry/contents";

#[derive(Deserialize)]
struct GithubContent {
    name: String,
}

pub async fn search_registry(crate_name: &str) -> Option<String> {
    let url = format!("{}/{}", REGISTRY_API, crate_name);
    let client = reqwest::Client::new();
    let res = client.get(&url)
        .header("User-Agent", "healdep")
        .header("Accept", "application/vnd.github.v3+json")
        .send().await.ok()?;
    if res.status().is_success() {
        let contents: Vec<GithubContent> = res.json().await.ok()?;
        if let Some(folder) = contents.first() {
            // Возвращаем имя первого shim-крейта как индикатор наличия решения
            return Some(folder.name.clone());
        }
    }
    None
}
