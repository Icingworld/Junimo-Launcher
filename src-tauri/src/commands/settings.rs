use tauri::command;

use crate::commands::mods;
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
    let old_row = sqlx::query_scalar::<_, Option<String>>(
        r#"
        SELECT value
        FROM t_settings
        WHERE key = 'game_path'
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    let old_norm = old_row
        .flatten()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let normalized = path.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() { None } else { Some(t) }
    });

    let path_changed = match (&old_norm, &normalized) {
        (Some(a), Some(b)) => a != b,
        (Some(_), None) => true,
        _ => false,
    };

    if path_changed && old_norm.is_some() {
        mods::disable_all_enabled_mods(&state).await?;
    }

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
