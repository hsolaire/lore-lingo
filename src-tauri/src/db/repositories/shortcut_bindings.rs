use crate::{
    db::{connection::Database, DatabaseError},
    domain::settings::ShortcutBindingSummary,
};

pub fn list(database: &Database) -> Result<Vec<ShortcutBindingSummary>, DatabaseError> {
    let connection = database.connect()?;
    let mut statement = connection.prepare(
        "SELECT action, input_kind, binding_key, binding_json, is_enabled
         FROM sys_shortcut_bindings ORDER BY action, input_kind",
    )?;
    let rows = statement.query_map([], |row| {
        let binding_json: Option<String> = row.get(3)?;
        let binding = binding_json
            .map(|value| super::json_value(3, value))
            .transpose()?;
        Ok(ShortcutBindingSummary {
            action: row.get(0)?,
            input_kind: row.get(1)?,
            binding_key: row.get(2)?,
            binding,
            is_enabled: row.get::<_, i64>(4)? != 0,
        })
    })?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

pub fn revision(database: &Database) -> Result<u64, DatabaseError> {
    let connection = database.connect()?;
    let revision: i64 = connection.query_row(
        "SELECT COALESCE(MAX(updated_at), 0) FROM sys_shortcut_bindings",
        [],
        |row| row.get(0),
    )?;
    Ok(revision.max(0) as u64)
}
