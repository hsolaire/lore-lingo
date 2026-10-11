pub mod app_preferences;
pub mod capture_settings;
pub mod model_profiles;
pub mod ocr_settings;
pub mod shortcut_bindings;
pub mod subtitle_library;
pub mod translation_cache;
pub mod translation_config;
pub mod translation_history;
pub mod translation_sessions;
pub mod translation_settings;
pub mod window_states;

use rusqlite::{types::Type, Error};

pub(crate) fn json_value(index: usize, value: String) -> rusqlite::Result<serde_json::Value> {
    serde_json::from_str(&value)
        .map_err(|error| Error::FromSqlConversionFailure(index, Type::Text, Box::new(error)))
}

pub(crate) fn json_strings(index: usize, value: String) -> rusqlite::Result<Vec<String>> {
    serde_json::from_str(&value)
        .map_err(|error| Error::FromSqlConversionFailure(index, Type::Text, Box::new(error)))
}
