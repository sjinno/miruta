// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod anime;
use anime::*;

mod jikan;
use jikan::*;

mod window_state;
use tauri::Manager;
use window_state::*;

use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite:miruta.db").await?;

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
        .setup(|app| {
            if std::env::var("TAURI_DEV").is_ok() {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
