use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::sync::Mutex;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimeDetail {
    pub(crate) id: i64,
    pub(crate) image_url: String,
    pub(crate) small_image_url: String,
    pub(crate) large_image_url: String,
    pub(crate) title: String,
    pub(crate) title_english: Option<String>,
    pub(crate) title_japanese: Option<String>,
    pub(crate) rating: Option<String>,
    pub(crate) synopsis: Option<String>,
    pub(crate) score: Option<f64>,
    pub(crate) scored_by: Option<i64>,
    pub(crate) rank: Option<i64>,
    pub(crate) year: Option<i64>,
    pub(crate) to_watch: Option<bool>,
    pub(crate) watched: Option<bool>,
    pub(crate) my_score: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeProps {
    watched: Option<bool>,
    to_watch: Option<bool>,
    my_score: Option<i64>,
}

#[tauri::command]
pub async fn upsert_anime(
    pool: tauri::State<'_, Arc<Mutex<SqlitePool>>>,
    anime: AnimeDetail,
    props: AnimeProps,
) -> Result<(), String> {
    let pool = pool.lock().await;

    let AnimeDetail {
        id,
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
        ..
    } = anime;

    let AnimeProps {
        watched,
        to_watch,
        my_score,
    } = props;

    sqlx::query!(
        "INSERT INTO anime (
            id,
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
            to_watch,
            watched,
            my_score
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
        ON CONFLICT(id) DO UPDATE SET to_watch = COALESCE(excluded.to_watch, anime.to_watch), watched = COALESCE(excluded.watched, anime.watched), my_score = COALESCE(excluded.my_score, anime.my_score)", 
        id,
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
        to_watch,
        watched,
        my_score
    )
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_anime(
    pool: tauri::State<'_, Arc<Mutex<SqlitePool>>>,
    id: i32,
) -> Result<(), String> {
    let pool = pool.lock().await;

    sqlx::query!("DELETE FROM anime WHERE id = ?", id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_watch_and_watched(
    pool: tauri::State<'_, Arc<Mutex<SqlitePool>>>,
) -> Result<Vec<AnimeDetail>, String> {
    let pool = pool.lock().await;

    let rows = sqlx::query!("SELECT * FROM anime WHERE to_watch = 1 OR watched = 1")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let anime = rows
        .iter()
        .map(|r| AnimeDetail {
            id: r.id,
            image_url: r.image_url.clone(),
            small_image_url: r.small_image_url.clone(),
            large_image_url: r.large_image_url.clone(),
            title: r.title.clone(),
            title_english: r.title_english.clone(),
            title_japanese: r.title_japanese.clone(),
            rating: r.rating.clone(),
            synopsis: r.synopsis.clone(),
            score: r.score,
            scored_by: r.scored_by,
            rank: r.rank,
            year: r.year,
            to_watch: r.to_watch,
            watched: r.watched,
            my_score: r.my_score,
        })
        .collect();

    Ok(anime)
}
