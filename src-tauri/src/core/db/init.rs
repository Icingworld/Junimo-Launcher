//! 初始化数据库结构和数据

use sqlx::SqlitePool;


/// 初始化数据库结构和数据
/// 
/// # 参数
/// 
/// - `pool`: 数据库连接池
/// 
/// # 返回
/// 
/// - `Result<(), sqlx::Error>`: 初始化结果
/// 
/// # 错误
/// 
/// - `sqlx::Error`: 数据库错误
/// 
/// # 示例
/// 
/// ```rust
/// let pool = init_sqlite_pool("data.db", 10).await?;
/// init_db(&pool).await?;
/// ```
/// 
/// # 注意事项
/// 
/// - 该函数会初始化数据库结构和数据，请确保在应用程序启动时调用
/// 
pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // 1. 简单配置表，用于存储游戏路径等信息
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS t_settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );
        "#
    )
    .execute(pool)
    .await?;

    // 2. 模组表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS t_mods (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            unique_id TEXT NOT NULL,
            version TEXT NOT NULL,

            name TEXT NOT NULL,
            author TEXT,
            description TEXT,
            entry_dll TEXT,
            minimum_api_version TEXT,
            update_keys TEXT,

            enabled BOOLEAN NOT NULL DEFAULT 0,
            source_folder_name TEXT NOT NULL,
            storage_path TEXT NOT NULL,
            created_at INTEGER NOT NULL,

            UNIQUE(unique_id, version)
        );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
