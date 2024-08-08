use serde::{Deserialize, Serialize};
use ts_rs::TS;
use urlencoding::encode;

use crate::AnimeDetail;

const JIKAN_API_URL: &str = "https://api.jikan.moe/v4/anime";

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimeSearchResult {
    pagination: Pagination,
    data: Vec<Anime>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimeList {
    pagination: Pagination,
    data: Vec<AnimeDetail>,
}

impl From<AnimeSearchResult> for AnimeList {
    fn from(result: AnimeSearchResult) -> Self {
        let AnimeSearchResult {
            pagination,
            data: anime,
        } = result;
        let data = anime.into_iter().map(AnimeDetail::from).collect();
        AnimeList { pagination, data }
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Pagination {
    last_visible_page: u16,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub(crate) struct Anime {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    pub(crate) images: Images,
    pub(crate) title: String,
    pub(crate) title_english: Option<String>,
    pub(crate) title_japanese: Option<String>,
    pub(crate) rating: Option<String>,
    pub(crate) synopsis: Option<String>,
    pub(crate) score: Option<f64>,
    pub(crate) scored_by: Option<i64>,
    pub(crate) rank: Option<i64>,
    pub(crate) year: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub(crate) struct Images {
    pub(crate) webp: WebP,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WebP {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

// query anime
#[tauri::command]
pub async fn query_anime(name: &str, page: Option<u8>) -> Result<AnimeList, String> {
    let encoded = encode(&name);
    let page = page.unwrap_or(1);
    let url = format!("{}?sfw&q={}&page={}", JIKAN_API_URL, encoded, page);
    eprintln!("shohei - url {url}");
    let body: AnimeSearchResult = reqwest::get(url).await.unwrap().json().await.unwrap();
    Ok(body.into())
}

#[tauri::command]
pub async fn get_recommendations(id: i32) -> Result<AnimeList, String> {
    let url = format!("{}/{}/recommendations", JIKAN_API_URL, id);
    eprintln!("shohei - url {url}");
    let body: AnimeSearchResult = reqwest::get(url).await.unwrap().json().await.unwrap();
    Ok(body.into())
}

#[tauri::command]
pub async fn get_top_anime() -> Result<AnimeList, String> {
    let url = "https://api.jikan.moe/v4/top/anime?sfw&type=tv&limit=8";
    let body: AnimeSearchResult = reqwest::get(url).await.unwrap().json().await.unwrap();
    Ok(body.into())
}

impl From<Anime> for AnimeDetail {
    fn from(anime: Anime) -> Self {
        let Anime {
            mal_id: id,
            images,
            title,
            title_english,
            title_japanese,
            rating,
            synopsis,
            score,
            scored_by,
            rank,
            year,
            ..
        } = anime;
        let Images {
            webp:
                WebP {
                    image_url,
                    small_image_url,
                    large_image_url,
                },
        } = images;
        AnimeDetail {
            id: id as i64,
            image_url,
            small_image_url,
            large_image_url,
            title,
            title_english,
            title_japanese,
            rating,
            synopsis,
            score,
            scored_by,
            rank,
            year,
            to_watch: None,
            watched: None,
            my_score: None,
        }
    }
}
