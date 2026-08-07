use rusqlite::OptionalExtension;

use crate::{
    db::{connection::Database, DatabaseError},
    domain::{history::HistorySummary, translation::LatestTranslationSummary},
};

pub fn latest(database: &Database) -> Result<Option<LatestTranslationSummary>, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT id, source_text, translated_text, source_language, target_language,
                    result_source, status, created_at
             FROM tr_history ORDER BY created_at DESC, id DESC LIMIT 1",
            [],
            |row| {
                Ok(LatestTranslationSummary {
                    history_id: row.get(0)?,
                    source_text: row.get(1)?,
                    translated_text: row.get(2)?,
                    source_language: row.get(3)?,
                    target_language: row.get(4)?,
                    result_source: row.get(5)?,
                    status: row.get(6)?,
                    created_at: row.get(7)?,
                })
            },
        )
        .optional()
        .map_err(Into::into)
}

pub fn summary(database: &Database) -> Result<HistorySummary, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT COUNT(*),
                    COALESCE(SUM(CASE WHEN is_favorite = 1 THEN 1 ELSE 0 END), 0)
             FROM tr_history",
            [],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        )
        .map(|(total_count, favorite_count)| HistorySummary {
            total_count: total_count.max(0) as u64,
            favorite_count: favorite_count.max(0) as u64,
        })
        .map_err(Into::into)
}
