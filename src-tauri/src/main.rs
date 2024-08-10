// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod anime;
use anime::*;

mod jikan;
use jikan::*;

mod window_state;
use tauri::api::path;
use window_state::*;

use std::{fs, path::Path, sync::Arc};

use sqlx::{Error, SqlitePool};
use tokio::sync::Mutex;

async fn initialize_database(pool: &SqlitePool) -> Result<(), Error> {
    let create_window_state_table = r#"
    CREATE TABLE IF NOT EXISTS window_state (
        id INTEGER PRIMARY KEY,
        w INTEGER,
        h INTEGER,
        x INTEGER,
        y INTEGER
    );
    "#;

    let create_anime_table = r#"
    CREATE TABLE IF NOT EXISTS anime (
        id INTEGER PRIMARY KEY,
        image_url TEXT NOT NULL,
        small_image_url TEXT NOT NULL,
        large_image_url TEXT NOT NULL,
        title TEXT NOT NULL,
        title_english TEXT,
        title_japanese TEXT,
        rating TEXT,
        synopsis TEXT,
        score REAL DEFAULT 0,
        scored_by INTEGER,
        rank INTEGER DEFAULT 9999,
        year INTEGER DEFAULT 9999,
        to_watch BOOLEAN DEFAULT FALSE,
        watched BOOLEAN DEFAULT FALSE,
        watched_date TEXT DEFAULT '12-9999',
        my_score INTEGER DEFAULT 0
    );
    "#;

    sqlx::query(&create_window_state_table)
        .execute(pool)
        .await?;
    sqlx::query(&create_anime_table).execute(pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Some(data_dir) = path::data_dir() else {
        return Ok(());
    };
    let Some(data_dir_) = data_dir.to_str() else {
        return Ok(());
    };

    let db_path = format!("{}/Miruta/db/miruta.db", data_dir_.to_string(),);
    let parent = Path::new(&db_path).parent().unwrap();

    if parent.exists() {
        if !Path::new(&db_path).exists() {
            fs::File::create(&db_path).unwrap();
        }
    } else {
        fs::create_dir_all(&parent).unwrap();
    }

    let database_url = format!("sqlite:{}", db_path);

    let pool = SqlitePool::connect(&database_url).await?;

    initialize_database(&pool).await?;

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(pool)))
        .invoke_handler(tauri::generate_handler![
            get_window_state,
            save_window_state,
            query_anime,
            upsert_anime,
            delete_anime,
            get_watch_and_watched,
            get_recommendations,
            get_top_anime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
