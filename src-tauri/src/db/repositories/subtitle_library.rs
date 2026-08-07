use crate::db::{connection::Database, DatabaseError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubtitleLibraryCounts {
    pub file_count: u64,
    pub ready_count: u64,
    pub error_count: u64,
}

pub fn counts(database: &Database) -> Result<SubtitleLibraryCounts, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT COUNT(*),
                    COALESCE(SUM(CASE WHEN index_status = 'ready' THEN 1 ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN index_status = 'error' THEN 1 ELSE 0 END), 0)
             FROM sub_files",
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .map(
            |(file_count, ready_count, error_count)| SubtitleLibraryCounts {
                file_count: file_count.max(0) as u64,
                ready_count: ready_count.max(0) as u64,
                error_count: error_count.max(0) as u64,
            },
        )
        .map_err(Into::into)
}
