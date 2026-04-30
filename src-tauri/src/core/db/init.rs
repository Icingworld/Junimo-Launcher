//! 初始化数据库结构和数据

use crate::core::db::init_sqlite_pool;


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

    // 2. 模组分组表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS t_mod_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    // 3. 模组表，存储模组中 manifest.json 文件的内容
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS t_mods (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            author TEXT NOT NULL,
            version TEXT NOT NULL,
            description TEXT NOT NULL,
            unique_id TEXT NOT NULL,
            entry_dll TEXT NOT NULL,
            minimum_api_version TEXT NOT NULL,
            update_keys TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    // 4. 模组分组与模组的关系表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS t_mod_group_mods (
            group_id INTEGER NOT NULL,
            mod_id INTEGER NOT NULL,
            is_activated BOOLEAN NOT NULL DEFAULT TRUE,
            PRIMARY KEY (group_id, mod_id)
        );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
