use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::sync::Mutex;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WindowState {
    w: i32,
    h: i32,
    x: i32,
    y: i32,
}

#[tauri::command]
pub async fn get_window_state(
    pool: tauri::State<'_, Arc<Mutex<SqlitePool>>>,
) -> Result<WindowState, String> {
    let pool = pool.lock().await;
    let row = sqlx::query!("SELECT w, h, x, y FROM window_state WHERE id = 1")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let w = row.w.unwrap_or(800) as i32;
    let h = row.h.unwrap_or(600) as i32;
    let x = row.x.unwrap_or_default() as i32;
    let y = row.y.unwrap_or_default() as i32;

    Ok(WindowState { w, h, x, y })
}

#[tauri::command]
pub async fn save_window_state(
    pool: tauri::State<'_, Arc<Mutex<SqlitePool>>>,
    state: WindowState,
) -> Result<(), String> {
    let pool = pool.lock().await;
    sqlx::query!(
        "INSERT INTO window_state (id, w, h, x, y) VALUES (1, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET w = excluded.w, h = excluded.h, x = excluded.x, y = excluded.y",
        state.w,
        state.h,
        state.x,
        state.y
    )
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
