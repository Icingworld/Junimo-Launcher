//! 核心数据库模块

pub mod db;
pub mod init;

pub use db::init_sqlite_pool;
