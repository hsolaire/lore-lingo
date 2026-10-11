//! 单次翻译配置仓储；本模块的传输对象不包含密钥字段。

use rusqlite::{params, TransactionBehavior};

use crate::db::{connection::Database, DatabaseError};

pub struct StoredTranslationConfig {
    pub endpoint: String,
    pub model: String,
    pub secret_ref: Option<String>,
    pub source_language: String,
    pub target_language: String,
}

pub fn load(database: &Database) -> Result<StoredTranslationConfig, DatabaseError> {
    database
        .connect()?
        .query_row(
            "SELECT COALESCE(p.api_base, ''), COALESCE(p.model_name, ''), p.secret_ref,
                s.source_language, s.target_language
         FROM sys_translation_settings s
         LEFT JOIN sys_model_profiles p ON p.id = s.default_model_profile_id AND p.is_enabled = 1
         WHERE s.id = 1",
            [],
            |row| {
                Ok(StoredTranslationConfig {
                    endpoint: row.get(0)?,
                    model: row.get(1)?,
                    secret_ref: row.get(2)?,
                    source_language: row.get(3)?,
                    target_language: row.get(4)?,
                })
            },
        )
        .map_err(Into::into)
}

pub fn save(database: &Database, config: &StoredTranslationConfig) -> Result<(), DatabaseError> {
    // 步骤 1. 在同一事务中读取默认配置，保留未编辑的模型选项和翻译行为。
    let mut connection = database.connect()?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let profile_id: Option<i64> = transaction.query_row(
        "SELECT default_model_profile_id FROM sys_translation_settings WHERE id = 1",
        [],
        |row| row.get(0),
    )?;
    // 步骤 2. 更新既有默认模型或建立配置，只写密钥引用。
    let profile_id = if let Some(id) = profile_id {
        let changed = transaction.execute(
            "UPDATE sys_model_profiles SET provider = 'openai_compatible', model_name = ?1,
             api_base = ?2, secret_ref = ?3, is_enabled = 1,
             updated_at = MAX(updated_at + 1, CAST(strftime('%s','now') AS INTEGER) * 1000) WHERE id = ?4",
            params![config.model, config.endpoint, config.secret_ref, id],
        )?;
        if changed != 1 {
            return Err(DatabaseError::InvalidData("默认模型不存在".into()));
        }
        id
    } else {
        transaction.execute(
            "INSERT INTO sys_model_profiles(name, provider, model_name, api_base, secret_ref)
             VALUES (?1, 'openai_compatible', ?2, ?3, ?4)",
            params![
                format!("翻译服务 {}", uuid::Uuid::new_v4()),
                config.model,
                config.endpoint,
                config.secret_ref
            ],
        )?;
        transaction.last_insert_rowid()
    };
    // 步骤 3. 同时提交默认模型指针和语言方向，任一写入失败整体回滚。
    transaction.execute(
        "UPDATE sys_translation_settings SET default_model_profile_id = ?1, source_language = ?2,
         target_language = ?3, updated_at = MAX(updated_at + 1, CAST(strftime('%s','now') AS INTEGER) * 1000)
         WHERE id = 1",
        params![profile_id, config.source_language, config.target_language],
    )?;
    transaction.commit().map_err(Into::into)
}

pub fn secret_is_referenced(database: &Database, reference: &str) -> Result<bool, DatabaseError> {
    database
        .connect()?
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sys_model_profiles WHERE secret_ref = ?1)",
            [reference],
            |row| row.get(0),
        )
        .map_err(Into::into)
}
