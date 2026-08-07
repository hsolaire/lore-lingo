pub mod connection;
pub mod migrations;
pub mod repositories;

use std::{fmt, io};

#[derive(Debug)]
pub enum DatabaseError {
    Sqlite(rusqlite::Error),
    Io(io::Error),
    Migration(String),
    InvalidData(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(formatter, "SQLite 操作失败：{error}"),
            Self::Io(error) => write!(formatter, "数据库文件操作失败：{error}"),
            Self::Migration(message) => write!(formatter, "数据库迁移失败：{message}"),
            Self::InvalidData(message) => write!(formatter, "数据库内容无效：{message}"),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<rusqlite::Error> for DatabaseError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<io::Error> for DatabaseError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
