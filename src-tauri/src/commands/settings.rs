use tauri::command;

use crate::AppState;

#[command]
pub async fn settings_get_game_path(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    let row = sqlx::query_scalar::<_, Option<String>>(
        r#"
        SELECT value
        FROM t_settings
        WHERE key = 'game_path'
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row.flatten().filter(|s| !s.trim().is_empty()))
}

#[command]
pub async fn settings_set_game_path(
    state: tauri::State<'_, AppState>,
    path: Option<String>,
) -> Result<(), String> {
    let normalized = path.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() { None } else { Some(t) }
    });

    // Upsert
    sqlx::query(
        r#"
        INSERT INTO t_settings (key, value)
        VALUES ('game_path', ?1)
        ON CONFLICT(key) DO UPDATE SET value = excluded.value
        "#,
    )
    .bind(normalized)
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

