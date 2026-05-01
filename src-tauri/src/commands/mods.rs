use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, QueryBuilder, Row};
use tauri::command;

use crate::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Manifest {
    name: String,
    #[serde(default)]
    author: Option<String>,
    version: String,
    #[serde(default)]
    description: Option<String>,
    unique_id: String,
    #[serde(default)]
    entry_dll: Option<String>,
    #[serde(default)]
    minimum_api_version: Option<String>,
    #[serde(default)]
    update_keys: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ModDto {
    pub id: i64,
    pub unique_id: String,
    pub version: String,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub enabled: bool,
    pub source_folder_name: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct PagedMods {
    pub total: i64,
    pub items: Vec<ModDto>,
}

fn file_name_string(p: &Path) -> Result<String, String> {
    p.file_name()
        .and_then(OsStr::to_str)
        .map(|s| s.to_string())
        .ok_or_else(|| "无法获取文件夹名称".to_string())
}

fn sanitize_path_segment(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        let ok = ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.';
        out.push(if ok { ch } else { '_' });
    }
    if out.is_empty() { "_".to_string() } else { out }
}

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    if !src.is_dir() {
        return Err("选择的路径不是文件夹".to_string());
    }
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;

    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_type = entry.file_type().map_err(|e| e.to_string())?;
        let from = entry.path();
        let to = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else if file_type.is_file() {
            fs::copy(&from, &to).map_err(|e| e.to_string())?;
        } else {
            // 跳过 symlink 等非常规条目，避免循环/权限问题
        }
    }
    Ok(())
}

async fn get_game_path(state: &AppState) -> Result<PathBuf, String> {
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

    let Some(path) = row.flatten().filter(|s| !s.trim().is_empty()) else {
        return Err("尚未在设置中选择游戏目录（game_path）".to_string());
    };
    Ok(PathBuf::from(path))
}

fn remove_link_path(link_path: &Path) -> Result<(), String> {
    // 使用 symlink_metadata：即使链接断了也能识别出来
    if fs::symlink_metadata(link_path).is_err() {
        return Ok(());
    }

    // 先尝试按目录删除（junction 通常表现为目录）
    if fs::remove_dir_all(link_path).is_ok() {
        return Ok(());
    }
    // 再尝试按文件删除（unix symlink）
    fs::remove_file(link_path).map_err(|e| e.to_string())
}

#[cfg(windows)]
fn create_dir_link(link_path: &Path, target_path: &Path) -> Result<(), String> {
    // mklink 是 cmd 内建命令，需通过 cmd /C 调用
    let link = link_path
        .to_str()
        .ok_or_else(|| "link_path 非法".to_string())?;
    let target = target_path
        .to_str()
        .ok_or_else(|| "target_path 非法".to_string())?;

    let cmd_str = format!(r#"mklink /J "{}" "{}""#, link, target);
    let status = std::process::Command::new("cmd")
        .args(["/C", &cmd_str])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("创建 junction 失败（mklink /J）".to_string())
    }
}

#[cfg(unix)]
fn create_dir_link(link_path: &Path, target_path: &Path) -> Result<(), String> {
    std::os::unix::fs::symlink(target_path, link_path).map_err(|e| e.to_string())
}

async fn set_enabled_internal(
    state: &AppState,
    mod_id: i64,
    enabled: bool,
) -> Result<(), String> {
    let row = sqlx::query(
        r#"
        SELECT id, enabled, source_folder_name, storage_path
        FROM t_mods
        WHERE id = ?1
        "#,
    )
    .bind(mod_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    let Some(row) = row else {
        return Err("模组不存在".to_string());
    };

    let current_enabled: i64 = row.get("enabled");
    let source_folder_name: String = row.get("source_folder_name");
    let storage_path: String = row.get("storage_path");

    if (current_enabled != 0) == enabled {
        return Ok(());
    }

    let game_path = get_game_path(state).await?;
    let game_mods_dir = game_path.join("Mods");
    fs::create_dir_all(&game_mods_dir).map_err(|e| e.to_string())?;

    let link_path = game_mods_dir.join(&source_folder_name);
    let target_path = PathBuf::from(storage_path);

    if enabled {
        // 重建链接
        let _ = remove_link_path(&link_path);
        create_dir_link(&link_path, &target_path)?;
    } else {
        // 删除链接（不存在也视为用户手动处理）
        let _ = remove_link_path(&link_path);
    }

    sqlx::query(
        r#"
        UPDATE t_mods
        SET enabled = ?2
        WHERE id = ?1
        "#,
    )
    .bind(mod_id)
    .bind(if enabled { 1i64 } else { 0i64 })
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub async fn mods_list(
    state: tauri::State<'_, AppState>,
    page: u32,
    page_size: u32,
    name_like: Option<String>,
    enabled_filter: Option<i32>, // -1/None: all, 1: enabled, 0: disabled
) -> Result<PagedMods, String> {
    let page = page.max(1);
    let page_size = page_size.clamp(1, 100);
    let offset = ((page - 1) * page_size) as i64;

    let name_like = name_like.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() { None } else { Some(t) }
    });

    let enabled_filter = enabled_filter.and_then(|v| match v {
        0 | 1 => Some(v),
        _ => None,
    });

    // total
    let mut qb_total: QueryBuilder<sqlx::Sqlite> =
        QueryBuilder::new("SELECT COUNT(1) AS cnt FROM t_mods");
    let mut has_where = false;
    if name_like.is_some() || enabled_filter.is_some() {
        qb_total.push(" WHERE ");
        has_where = true;
    }
    if let Some(name) = &name_like {
        qb_total.push(" name LIKE ");
        qb_total.push_bind(format!("%{}%", name));
    }
    if let Some(filter) = enabled_filter {
        if has_where && name_like.is_some() {
            qb_total.push(" AND ");
        }
        qb_total.push(" enabled = ");
        qb_total.push_bind(filter);
    }
    let total: i64 = qb_total
        .build()
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())?
        .try_get::<i64, _>("cnt")
        .map_err(|e| e.to_string())?;

    // items
    let mut qb: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(
        r#"
        SELECT
          id, unique_id, version, name, author, description, enabled, source_folder_name, created_at
        FROM t_mods
        "#,
    );

    let mut where_started = false;
    if name_like.is_some() || enabled_filter.is_some() {
        qb.push(" WHERE ");
        where_started = true;
    }
    if let Some(name) = &name_like {
        qb.push(" name LIKE ");
        qb.push_bind(format!("%{}%", name));
    }
    if let Some(filter) = enabled_filter {
        if where_started && name_like.is_some() {
            qb.push(" AND ");
        }
        qb.push(" enabled = ");
        qb.push_bind(filter);
    }

    qb.push(" ORDER BY created_at DESC, id DESC ");
    qb.push(" LIMIT ");
    qb.push_bind(page_size as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset);

    let rows: Vec<SqliteRow> = qb
        .build()
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let items = rows
        .into_iter()
        .map(|r| ModDto {
            id: r.get("id"),
            unique_id: r.get("unique_id"),
            version: r.get("version"),
            name: r.get("name"),
            author: r.get("author"),
            description: r.get("description"),
            enabled: (r.get::<i64, _>("enabled")) != 0,
            source_folder_name: r.get("source_folder_name"),
            created_at: r.get("created_at"),
        })
        .collect();

    Ok(PagedMods { total, items })
}

#[command]
pub async fn mods_add(
    state: tauri::State<'_, AppState>,
    folder_path: String,
) -> Result<ModDto, String> {
    let folder = PathBuf::from(folder_path);
    if !folder.is_dir() {
        return Err("选择的路径不是文件夹".to_string());
    }

    let source_folder_name = file_name_string(&folder)?;
    let manifest_path = folder.join("manifest.json");
    let manifest_content = fs::read_to_string(&manifest_path)
        .map_err(|_| "读取 manifest.json 失败，请确认该文件存在".to_string())?;
    let manifest: Manifest =
        serde_json::from_str(&manifest_content).map_err(|e| format!("解析 manifest.json 失败: {e}"))?;

    // 判重：UniqueID + Version
    let exists = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(1)
        FROM t_mods
        WHERE unique_id = ?1 AND version = ?2
        "#,
    )
    .bind(&manifest.unique_id)
    .bind(&manifest.version)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    if exists > 0 {
        return Err("该模组版本已存在（UniqueID + Version 重复）".to_string());
    }

    let unique_seg = sanitize_path_segment(&manifest.unique_id);
    let version_seg = sanitize_path_segment(&manifest.version);
    let storage_dir = state
        .app_mods_dir
        .join(unique_seg)
        .join(version_seg)
        .join(sanitize_path_segment(&source_folder_name));

    if storage_dir.exists() {
        return Err("目标存储目录已存在，请先删除后重试".to_string());
    }

    copy_dir_all(&folder, &storage_dir)?;

    let created_at = now_ts();
    let update_keys_str = manifest
        .update_keys
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "null".to_string());

    let result = sqlx::query(
        r#"
        INSERT INTO t_mods (
          unique_id, version,
          name, author, description, entry_dll, minimum_api_version, update_keys,
          enabled, source_folder_name, storage_path, created_at
        )
        VALUES (
          ?1, ?2,
          ?3, ?4, ?5, ?6, ?7, ?8,
          0, ?9, ?10, ?11
        )
        "#,
    )
    .bind(&manifest.unique_id)
    .bind(&manifest.version)
    .bind(&manifest.name)
    .bind(manifest.author.as_deref())
    .bind(manifest.description.as_deref())
    .bind(manifest.entry_dll.as_deref())
    .bind(manifest.minimum_api_version.as_deref())
    .bind(update_keys_str)
    .bind(&source_folder_name)
    .bind(storage_dir.to_string_lossy().to_string())
    .bind(created_at)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        // 尝试回滚文件复制
        let _ = fs::remove_dir_all(&storage_dir);
        e.to_string()
    })?;

    let id = result.last_insert_rowid();

    Ok(ModDto {
        id,
        unique_id: manifest.unique_id,
        version: manifest.version,
        name: manifest.name,
        author: manifest.author,
        description: manifest.description,
        enabled: false,
        source_folder_name,
        created_at,
    })
}

#[command]
pub async fn mods_delete(state: tauri::State<'_, AppState>, mod_id: i64) -> Result<(), String> {
    let row = sqlx::query(
        r#"
        SELECT id, enabled, source_folder_name, storage_path
        FROM t_mods
        WHERE id = ?1
        "#,
    )
    .bind(mod_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    let Some(row) = row else {
        return Ok(());
    };

    let enabled: i64 = row.get("enabled");
    let storage_path: String = row.get("storage_path");

    if enabled != 0 {
        // 尝试先禁用（如果用户没设置 game_path，会报错；这里我们允许继续删除 DB/存储）
        let _ = set_enabled_internal(&state, mod_id, false).await;
    }

    sqlx::query("DELETE FROM t_mods WHERE id = ?1")
        .bind(mod_id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let _ = fs::remove_dir_all(PathBuf::from(storage_path));
    Ok(())
}

#[command]
pub async fn mods_set_enabled(
    state: tauri::State<'_, AppState>,
    mod_id: i64,
    enabled: bool,
) -> Result<(), String> {
    set_enabled_internal(&state, mod_id, enabled).await
}

