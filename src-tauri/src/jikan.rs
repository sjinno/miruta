use serde::Deserialize;
// use anyhow::Result;
use urlencoding::encode;

const JIKAN_API_URL: &str = "https://api.jikan.moe/v4/anime";

#[derive(Debug, Deserialize)]
struct AnimeSearch {
    data: Vec<Anime>,
}

#[derive(Debug, Deserialize)]
struct Anime {
    mal_id: usize,
    url: String,
    images: Images,
    title: String,
    title_japanese: String,
    score: Option<f32>,
    scored_by: Option<usize>,
    rank: Option<usize>,
    year: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct Images {
    webp: WebP,
}

#[derive(Debug, Deserialize)]
struct WebP {
    image_url: String,
    large_image_url: String,
}

// query anime
#[tauri::command]
pub async fn query_anime(query: &str) -> Result<String, String> {
    let encoded = encode(&query);
    let url = format!("{}?q={}", JIKAN_API_URL, encoded);
    eprintln!("shohei - url {url}");

    let body: AnimeSearch = reqwest::get(url).await.unwrap().json().await.unwrap();
    println!("body = {body:#?}");

    Ok(String::from("Hello, World!"))
}

// format anime results
