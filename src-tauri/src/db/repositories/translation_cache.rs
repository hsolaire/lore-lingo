use crate::db::{connection::Database, DatabaseError};

pub fn count(database: &Database) -> Result<u64, DatabaseError> {
    let connection = database.connect()?;
    let count: i64 = connection
        .query_row("SELECT COUNT(*) FROM tr_cache", [], |row| row.get(0))
        .map_err(DatabaseError::from)?;
    Ok(count.max(0) as u64)
}
