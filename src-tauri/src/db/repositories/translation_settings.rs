use crate::{
    db::{connection::Database, DatabaseError},
    domain::settings::{TranslationMode, TranslationSettingsSummary},
};

pub fn get(database: &Database) -> Result<TranslationSettingsSummary, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT default_model_profile_id, mode, auto_interval_ms, source_language,
                    target_language, request_timeout_ms, prefer_subtitles, save_history,
                    history_retention_days
             FROM sys_translation_settings WHERE id = 1",
            [],
            |row| {
                let mode = match row.get::<_, String>(1)?.as_str() {
                    "manual" => TranslationMode::Manual,
                    "auto" => TranslationMode::Auto,
                    value => return Err(rusqlite::Error::InvalidParameterName(value.into())),
                };
                Ok(TranslationSettingsSummary {
                    default_model_profile_id: row.get(0)?,
                    mode,
                    auto_interval_ms: row.get(2)?,
                    source_language: row.get(3)?,
                    target_language: row.get(4)?,
                    request_timeout_ms: row.get(5)?,
                    prefer_subtitles: row.get::<_, i64>(6)? != 0,
                    save_history: row.get::<_, i64>(7)? != 0,
                    history_retention_days: row.get(8)?,
                })
            },
        )
        .map_err(Into::into)
}

pub fn revision(database: &Database) -> Result<u64, DatabaseError> {
    let connection = database.connect()?;
    let revision: i64 = connection.query_row(
        "SELECT updated_at FROM sys_translation_settings WHERE id = 1",
        [],
        |row| row.get(0),
    )?;
    Ok(revision.max(0) as u64)
}
