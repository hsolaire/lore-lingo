use crate::{
    db::{connection::Database, DatabaseError},
    domain::settings::OcrSettingsSummary,
};

use super::{json_strings, json_value};

pub fn get(database: &Database) -> Result<OcrSettingsSummary, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT engine, api_base, secret_ref, recognition_languages_json, sample_fps,
                    min_confidence, deduplicate_frames, options_json
             FROM sys_ocr_settings WHERE id = 1",
            [],
            |row| {
                let secret_ref: Option<String> = row.get(2)?;
                Ok(OcrSettingsSummary {
                    engine: row.get(0)?,
                    api_base: row.get(1)?,
                    has_secret: secret_ref.is_some(),
                    recognition_languages: json_strings(3, row.get(3)?)?,
                    sample_fps: row.get(4)?,
                    min_confidence: row.get(5)?,
                    deduplicate_frames: row.get::<_, i64>(6)? != 0,
                    options: json_value(7, row.get(7)?)?,
                })
            },
        )
        .map_err(Into::into)
}

pub fn revision(database: &Database) -> Result<u64, DatabaseError> {
    let connection = database.connect()?;
    let revision: i64 = connection.query_row(
        "SELECT updated_at FROM sys_ocr_settings WHERE id = 1",
        [],
        |row| row.get(0),
    )?;
    Ok(revision.max(0) as u64)
}
