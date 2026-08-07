use rusqlite::OptionalExtension;

use crate::{
    db::{connection::Database, DatabaseError},
    domain::settings::ModelProfileSummary,
};

use super::json_value;

pub fn list(database: &Database) -> Result<Vec<ModelProfileSummary>, DatabaseError> {
    let connection = database.connect()?;
    let mut statement = connection.prepare(
        "SELECT id, name, provider, model_name, api_base, secret_ref, options_json, is_enabled
         FROM sys_model_profiles ORDER BY name COLLATE NOCASE",
    )?;
    let rows = statement.query_map([], map_profile)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

pub fn default_profile(database: &Database) -> Result<Option<ModelProfileSummary>, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT p.id, p.name, p.provider, p.model_name, p.api_base, p.secret_ref, p.options_json, p.is_enabled
             FROM sys_translation_settings s
             JOIN sys_model_profiles p ON p.id = s.default_model_profile_id
             WHERE s.id = 1 AND p.is_enabled = 1",
            [],
            map_profile,
        )
        .optional()
        .map_err(Into::into)
}

pub fn revision(database: &Database) -> Result<u64, DatabaseError> {
    let connection = database.connect()?;
    let revision: i64 = connection.query_row(
        "SELECT COALESCE(MAX(updated_at), 0) FROM sys_model_profiles",
        [],
        |row| row.get(0),
    )?;
    Ok(revision.max(0) as u64)
}

fn map_profile(row: &rusqlite::Row<'_>) -> rusqlite::Result<ModelProfileSummary> {
    let secret_ref: Option<String> = row.get(5)?;
    Ok(ModelProfileSummary {
        id: row.get(0)?,
        name: row.get(1)?,
        provider: row.get(2)?,
        model_name: row.get(3)?,
        api_base: row.get(4)?,
        has_secret: secret_ref.is_some(),
        options: json_value(6, row.get(6)?)?,
        is_enabled: row.get::<_, i64>(7)? != 0,
    })
}
