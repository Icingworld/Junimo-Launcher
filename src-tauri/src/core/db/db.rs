//! 初始化 SQLite 连接池
//! 
//! 使用 sqlite3 作为数据库，使用 sqlx 作为数据库操作库


use std::path::Path;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool
};


/// 初始化 SQLite 连接池
/// 
/// # 参数
/// 
/// - `db_path`: 数据库文件路径
/// - `max_connections`: 最大连接数
/// 
/// # 返回
/// 
/// - `Result<SqlitePool, sqlx::Error>`: 连接池
/// 
/// # 错误
/// 
/// - `sqlx::Error`: 数据库错误
/// 
/// # 示例
/// 
/// ```rust
/// let pool = init_sqlite_pool("data.db", 10).await?;
/// ```
/// 
pub async fn init_sqlite_pool(db_path: &Path, max_connections: u32) -> Result<SqlitePool, sqlx::Error> {
  let options = SqliteConnectOptions::new()
    .filename(db_path)
    .create_if_missing(true)
    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
    .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
    .busy_timeout(std::time::Duration::from_secs(10));

  let pool = SqlitePoolOptions::new()
    .max_connections(max_connections)
    .connect_with(options)
    .await?;

  Ok(pool)
}
