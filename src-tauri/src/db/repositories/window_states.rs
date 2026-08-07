use crate::db::{connection::Database, DatabaseError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredWindowState {
    pub label: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub always_on_top: bool,
}

pub fn list(database: &Database) -> Result<Vec<StoredWindowState>, DatabaseError> {
    let connection = database.connect()?;
    let mut statement = connection.prepare(
        "SELECT window_label, x, y, width, height, always_on_top
         FROM sys_window_states ORDER BY window_label",
    )?;
    let rows = statement.query_map([], |row| {
        Ok(StoredWindowState {
            label: row.get(0)?,
            x: row.get(1)?,
            y: row.get(2)?,
            width: row.get(3)?,
            height: row.get(4)?,
            always_on_top: row.get::<_, i64>(5)? != 0,
        })
    })?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}
