use rusqlite::OptionalExtension;

use crate::{
    db::{connection::Database, DatabaseError},
    domain::translation::{ActiveSessionSummary, LiveSessionStatus},
};

pub fn active(database: &Database) -> Result<Option<ActiveSessionSummary>, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT id, mode, started_at FROM tr_sessions
             WHERE ended_at IS NULL ORDER BY started_at DESC LIMIT 1",
            [],
            |row| {
                Ok(ActiveSessionSummary {
                    session_id: row.get(0)?,
                    mode: row.get(1)?,
                    status: LiveSessionStatus::Running,
                    started_at: row.get(2)?,
                })
            },
        )
        .optional()
        .map_err(Into::into)
}

pub fn revision(database: &Database) -> Result<u64, DatabaseError> {
    let connection = database.connect()?;
    let revision: i64 = connection.query_row(
        "SELECT MAX(
                COALESCE((SELECT MAX(COALESCE(ended_at, started_at)) FROM tr_sessions), 0),
                COALESCE((SELECT MAX(updated_at) FROM tr_history), 0)
             )",
        [],
        |row| row.get(0),
    )?;
    Ok(revision.max(0) as u64)
}
