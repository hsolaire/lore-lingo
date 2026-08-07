use crate::{
    db::{connection::Database, DatabaseError},
    domain::settings::{CaptureMode, CaptureRegionSummary, CaptureSettingsSummary},
};

pub fn get(database: &Database) -> Result<CaptureSettingsSummary, DatabaseError> {
    let connection = database.connect()?;
    connection
        .query_row(
            "SELECT mode, region_x, region_y, region_width, region_height, app_selector_json
             FROM sys_capture_settings WHERE id = 1",
            [],
            |row| {
                let mode_value: String = row.get(0)?;
                let mode = match mode_value.as_str() {
                    "region" => CaptureMode::Region,
                    "window" => CaptureMode::Window,
                    "app" => CaptureMode::App,
                    "display" => CaptureMode::Display,
                    _ => return Err(rusqlite::Error::InvalidParameterName(mode_value)),
                };
                let region_values = (
                    row.get::<_, Option<i32>>(1)?,
                    row.get::<_, Option<i32>>(2)?,
                    row.get::<_, Option<u32>>(3)?,
                    row.get::<_, Option<u32>>(4)?,
                );
                let region = match region_values {
                    (Some(x), Some(y), Some(width), Some(height)) => Some(CaptureRegionSummary {
                        x,
                        y,
                        width,
                        height,
                    }),
                    _ => None,
                };
                let selector: Option<String> = row.get(5)?;
                let app_selector = selector
                    .map(|value| super::json_value(5, value))
                    .transpose()?;
                Ok(CaptureSettingsSummary {
                    mode,
                    region,
                    app_selector,
                })
            },
        )
        .map_err(Into::into)
}

pub fn revision(database: &Database) -> Result<u64, DatabaseError> {
    let connection = database.connect()?;
    let revision: i64 = connection.query_row(
        "SELECT updated_at FROM sys_capture_settings WHERE id = 1",
        [],
        |row| row.get(0),
    )?;
    Ok(revision.max(0) as u64)
}
