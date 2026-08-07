use rusqlite::params;

use crate::db::{connection::Database, DatabaseError};

pub fn put(database: &Database, key: &str, value: &serde_json::Value) -> Result<(), DatabaseError> {
    let connection = database.connect()?;
    connection.execute(
        "INSERT INTO sys_app_preferences(key, value_json) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json,
         updated_at = CAST(strftime('%s', 'now') AS INTEGER) * 1000",
        params![key, value.to_string()],
    )?;
    Ok(())
}

pub fn get(database: &Database, key: &str) -> Result<Option<serde_json::Value>, DatabaseError> {
    use rusqlite::OptionalExtension;

    let connection = database.connect()?;
    let value = connection
        .query_row(
            "SELECT value_json FROM sys_app_preferences WHERE key = ?1",
            [key],
            |row| row.get::<_, String>(0),
        )
        .optional()?;
    value
        .map(|value| {
            serde_json::from_str(&value)
                .map_err(|error| DatabaseError::InvalidData(error.to_string()))
        })
        .transpose()
}
