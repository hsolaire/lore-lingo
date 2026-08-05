use std::sync::Arc;

use crate::{
    db::{
        connection::Database,
        repositories::{
            capture_settings, model_profiles, ocr_settings, shortcut_bindings, translation_history,
            translation_sessions, translation_settings,
        },
    },
    domain::{
        app::{BootstrapSnapshot, PlatformCapabilities, WindowSummary},
        settings::SettingsSummary,
        translation::PublicError,
    },
};

#[derive(Clone)]
pub struct BootstrapService {
    database: Result<Arc<Database>, PublicError>,
}

impl BootstrapService {
    pub fn new(database: Result<Database, crate::db::DatabaseError>) -> Self {
        Self {
            database: database
                .map(Arc::new)
                .map_err(|_| PublicError::database_unavailable()),
        }
    }

    pub fn unavailable() -> Self {
        Self {
            database: Err(PublicError::database_unavailable()),
        }
    }

    pub fn snapshot(&self, windows: Vec<WindowSummary>) -> Result<BootstrapSnapshot, PublicError> {
        let database = self.database.as_ref().map_err(Clone::clone)?;

        // 步骤 1. 从职责明确的仓储加载启动所需的持久化快照
        let translation = translation_settings::get(database).map_err(map_database_error)?;
        let ocr = ocr_settings::get(database).map_err(map_database_error)?;
        let default_model =
            model_profiles::default_profile(database).map_err(map_database_error)?;
        let shortcuts = shortcut_bindings::list(database).map_err(map_database_error)?;
        let capture = capture_settings::get(database).map_err(map_database_error)?;
        let active_session = translation_sessions::active(database).map_err(map_database_error)?;
        let latest_result = translation_history::latest(database).map_err(map_database_error)?;
        let history = translation_history::summary(database).map_err(map_database_error)?;

        // 步骤 2. 计算可用于丢失事件后重新查询的单调数据修订号
        let settings_revision = [
            translation_settings::revision(database).map_err(map_database_error)?,
            ocr_settings::revision(database).map_err(map_database_error)?,
            model_profiles::revision(database).map_err(map_database_error)?,
            shortcut_bindings::revision(database).map_err(map_database_error)?,
        ]
        .into_iter()
        .max()
        .unwrap_or(0);
        let capture_revision = capture_settings::revision(database).map_err(map_database_error)?;
        let session_revision =
            translation_sessions::revision(database).map_err(map_database_error)?;

        // 步骤 3. 只组合平台中立 DTO，不向 Vue 暴露数据库或原生对象
        Ok(BootstrapSnapshot {
            platform: PlatformCapabilities::current(),
            windows,
            settings: SettingsSummary { translation, ocr },
            default_model,
            shortcuts,
            capture,
            active_session,
            latest_result,
            history,
            settings_revision,
            capture_revision,
            session_revision,
        })
    }
}

fn map_database_error(_: crate::db::DatabaseError) -> PublicError {
    PublicError::database_unavailable()
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    fn test_database() -> (Database, std::path::PathBuf) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("lore-lingo-bootstrap-{nonce}.sqlite3"));
        let database = Database::initialize_file(path.clone()).expect("初始化测试数据库");
        (database, path)
    }

    #[test]
    fn empty_database_produces_real_default_snapshot() {
        let (database, path) = test_database();
        let snapshot = BootstrapService::new(Ok(database))
            .snapshot(Vec::new())
            .expect("读取启动快照");
        assert_eq!(snapshot.settings.translation.target_language, "zh-CN");
        assert!(snapshot.default_model.is_none());
        assert!(snapshot.active_session.is_none());
        assert!(snapshot.latest_result.is_none());
        assert_eq!(snapshot.history.total_count, 0);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn default_model_is_loaded_and_delete_falls_back_to_none() {
        let (database, path) = test_database();
        let connection = database.connect().expect("打开测试数据库");
        connection
            .execute(
                "INSERT INTO sys_model_profiles(name, provider, model_name)
                 VALUES ('默认模型', 'openai_compatible', 'demo-model')",
                [],
            )
            .expect("插入默认模型");
        let profile_id = connection.last_insert_rowid();
        connection
            .execute(
                "UPDATE sys_translation_settings SET default_model_profile_id = ?1 WHERE id = 1",
                [profile_id],
            )
            .expect("设置默认模型");
        drop(connection);

        let service = BootstrapService::new(Ok(database.clone()));
        let configured = service.snapshot(Vec::new()).expect("读取已配置快照");
        assert_eq!(
            configured.default_model.map(|model| model.id),
            Some(profile_id)
        );

        database
            .connect()
            .expect("重新打开测试数据库")
            .execute("DELETE FROM sys_model_profiles WHERE id = ?1", [profile_id])
            .expect("删除默认模型");
        let deleted = service.snapshot(Vec::new()).expect("读取删除后的快照");
        assert!(deleted.default_model.is_none());

        drop(service);
        drop(database);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn database_failure_maps_to_stable_public_error() {
        let error = BootstrapService::unavailable()
            .snapshot(Vec::new())
            .expect_err("数据库失败必须拒绝快照");
        assert_eq!(error.code, "database_error");
        assert!(!error.message.contains('/'));
    }
}
