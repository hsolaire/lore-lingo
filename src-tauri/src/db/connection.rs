use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use rusqlite::{Connection, TransactionBehavior};

use super::{migrations::run_migrations, DatabaseError};

pub(super) const DATABASE_FILE_NAME: &str = "lore-lingo.sqlite3";
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Debug)]
pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn initialize(directory: &Path) -> Result<Self, DatabaseError> {
        fs::create_dir_all(directory)?;
        Self::initialize_file(directory.join(DATABASE_FILE_NAME))
    }

    pub fn initialize_file(path: PathBuf) -> Result<Self, DatabaseError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let database = Self { path };
        let mut connection = database.connect()?;
        prepare_database(&mut connection)?;
        Ok(database)
    }

    pub fn connect(&self) -> Result<Connection, DatabaseError> {
        let connection = Connection::open(&self.path)?;
        configure_connection(&connection)?;
        Ok(connection)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub(super) fn prepare_database(connection: &mut Connection) -> Result<(), DatabaseError> {
    run_migrations(connection)?;
    ensure_singleton_defaults(connection)?;
    verify_sqlite_capabilities(connection)
}

pub fn configure_connection(connection: &Connection) -> Result<(), DatabaseError> {
    connection.busy_timeout(BUSY_TIMEOUT)?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    Ok(())
}

fn ensure_singleton_defaults(connection: &mut Connection) -> Result<(), DatabaseError> {
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    transaction.execute_batch(
        "INSERT OR IGNORE INTO sys_translation_settings(id) VALUES (1);
         INSERT OR IGNORE INTO sys_ocr_settings(id) VALUES (1);
         INSERT OR IGNORE INTO sys_capture_settings(id) VALUES (1);",
    )?;
    transaction.commit()?;
    Ok(())
}

fn verify_sqlite_capabilities(connection: &Connection) -> Result<(), DatabaseError> {
    let json_valid: i64 = connection.query_row("SELECT json_valid('{}')", [], |row| row.get(0))?;
    if json_valid != 1 {
        return Err(DatabaseError::InvalidData(
            "当前 SQLite 未启用 JSON 支持".into(),
        ));
    }

    connection.execute_batch(
        "CREATE VIRTUAL TABLE temp.db_runtime_fts_probe USING fts5(content, tokenize='trigram');
         DROP TABLE temp.db_runtime_fts_probe;",
    )?;
    Ok(())
}
