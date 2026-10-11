//! 协调 SQLite 配置与系统密钥，保存成功才切换当前配置。

use std::sync::{Arc, Mutex};

use super::capture_translation::CaptureTranslationError;
use crate::{
    db::{
        connection::Database,
        repositories::translation_config::{self, StoredTranslationConfig},
        DatabaseError,
    },
    desktop::credentials::{CredentialStore, SystemCredentialStore},
    domain::captured_layout::TranslationConfig,
};

pub struct TranslationConfigService {
    database: Result<Database, CaptureTranslationError>,
    credentials: Arc<dyn CredentialStore>,
    // 此锁只序列化设置存储，不阻塞捕获、OCR 或网络任务。
    access: Mutex<()>,
}

impl TranslationConfigService {
    pub fn new(database: Result<Database, DatabaseError>) -> Self {
        Self {
            database: database.map_err(|_| CaptureTranslationError::DatabaseUnavailable),
            credentials: Arc::new(SystemCredentialStore),
            access: Mutex::new(()),
        }
    }

    pub fn load(&self) -> Result<TranslationConfig, CaptureTranslationError> {
        let _access = self
            .access
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        let stored = translation_config::load(self.database.as_ref().map_err(|error| *error)?)
            .map_err(|_| CaptureTranslationError::DatabaseUnavailable)?;
        let api_key = match &stored.secret_ref {
            Some(reference) => self
                .credentials
                .read(reference)
                .map_err(|_| CaptureTranslationError::CredentialUnavailable)?
                .unwrap_or_default(),
            None => String::new(),
        };
        Ok(TranslationConfig {
            endpoint: stored.endpoint,
            model: stored.model,
            api_key,
            source_language: stored.source_language,
            target_language: stored.target_language,
        })
    }

    pub fn save(
        &self,
        config: TranslationConfig,
    ) -> Result<TranslationConfig, CaptureTranslationError> {
        // 步骤 1. 校验完成后再访问存储，读取失败也不覆盖旧记录。
        let config = normalize_config(config)?;
        let _access = self
            .access
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        let database = self.database.as_ref().map_err(|error| *error)?;
        let old = translation_config::load(database)
            .map_err(|_| CaptureTranslationError::DatabaseUnavailable)?;
        let old_secret = if let Some(reference) = &old.secret_ref {
            self.credentials
                .read(reference)
                .map_err(|_| CaptureTranslationError::CredentialUnavailable)?
        } else {
            None
        };
        let changed_secret = old_secret.as_deref().unwrap_or_default() != config.api_key;
        // 步骤 2. 新密钥使用独立引用，验证可读后才允许数据库切换。
        let secret_ref = if config.api_key.is_empty() {
            None
        } else if !changed_secret {
            old.secret_ref.clone()
        } else {
            let reference = format!("translation-{}", uuid::Uuid::new_v4());
            let write = self
                .credentials
                .write(&reference, &config.api_key)
                .and_then(|()| {
                    if self.credentials.read(&reference)?.as_deref()
                        == Some(config.api_key.as_str())
                    {
                        Ok(())
                    } else {
                        Err(crate::desktop::credentials::CredentialError::Unavailable)
                    }
                });
            if write.is_err() {
                let _ = self.credentials.delete(&reference);
                return Err(CaptureTranslationError::CredentialUnavailable);
            }
            Some(reference)
        };
        let stored = StoredTranslationConfig {
            endpoint: config.endpoint.clone(),
            model: config.model.clone(),
            secret_ref: secret_ref.clone(),
            source_language: config.source_language.clone(),
            target_language: config.target_language.clone(),
        };
        // 步骤 3. 原子保存模型与语言，失败只清理本次新建凭据。
        if translation_config::save(database, &stored).is_err() {
            if changed_secret {
                if let Some(reference) = &secret_ref {
                    let _ = self.credentials.delete(reference);
                }
            }
            return Err(CaptureTranslationError::DatabaseUnavailable);
        }
        // 步骤 4. 提交后尽力清理不再引用的旧凭据；失败不撤销已提交配置。
        if let Some(reference) = old
            .secret_ref
            .filter(|old_ref| Some(old_ref) != secret_ref.as_ref())
        {
            if matches!(
                translation_config::secret_is_referenced(database, &reference),
                Ok(false)
            ) {
                let _ = self.credentials.delete(&reference);
            }
        }
        Ok(config)
    }
}

fn normalize_config(
    mut config: TranslationConfig,
) -> Result<TranslationConfig, CaptureTranslationError> {
    config.endpoint = config.endpoint.trim().into();
    config.model = config.model.trim().into();
    config.api_key = config.api_key.trim().into();
    config.source_language = config.source_language.trim().into();
    config.target_language = config.target_language.trim().into();
    if config.endpoint.len() > 2048
        || config.model.len() > 256
        || config.api_key.len() > 16384
        || config.source_language.len() > 128
        || config.target_language.len() > 128
        || config.source_language.is_empty()
        || config.target_language.is_empty()
        || config.target_language == "auto"
    {
        return Err(CaptureTranslationError::TranslationNotConfigured);
    }
    super::translation::validated_completion_url(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{desktop::credentials::CredentialError, services::bootstrap::BootstrapService};
    use std::{
        collections::HashMap,
        path::PathBuf,
        sync::atomic::{AtomicBool, Ordering},
    };

    #[derive(Default)]
    struct MemoryCredentials {
        values: Mutex<HashMap<String, String>>,
        fail_write: AtomicBool,
        fail_read: AtomicBool,
        fail_delete: AtomicBool,
    }
    impl CredentialStore for MemoryCredentials {
        fn read(&self, reference: &str) -> Result<Option<String>, CredentialError> {
            if self.fail_read.load(Ordering::Relaxed) {
                return Err(CredentialError::Unavailable);
            }
            Ok(self
                .values
                .lock()
                .expect("读取测试存储")
                .get(reference)
                .cloned())
        }
        fn write(&self, reference: &str, secret: &str) -> Result<(), CredentialError> {
            if self.fail_write.load(Ordering::Relaxed) {
                return Err(CredentialError::Unavailable);
            }
            self.values
                .lock()
                .expect("写入测试存储")
                .insert(reference.into(), secret.into());
            Ok(())
        }
        fn delete(&self, reference: &str) -> Result<(), CredentialError> {
            if self.fail_delete.load(Ordering::Relaxed) {
                return Err(CredentialError::Unavailable);
            }
            self.values.lock().expect("清理测试存储").remove(reference);
            Ok(())
        }
    }

    struct Harness {
        directory: PathBuf,
        database: Database,
        credentials: Arc<MemoryCredentials>,
    }
    impl Harness {
        fn new() -> Self {
            let directory =
                std::env::temp_dir().join(format!("lore-lingo-settings-{}", uuid::Uuid::new_v4()));
            Self {
                database: Database::initialize(&directory).expect("初始化测试数据库"),
                directory,
                credentials: Arc::new(MemoryCredentials::default()),
            }
        }
        fn service(&self) -> TranslationConfigService {
            TranslationConfigService {
                database: Ok(self.database.clone()),
                credentials: self.credentials.clone(),
                access: Mutex::new(()),
            }
        }
    }
    impl Drop for Harness {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.directory);
        }
    }
    fn local_config() -> TranslationConfig {
        TranslationConfig {
            endpoint: "http://127.0.0.1:11434/v1".into(),
            model: "test-model".into(),
            api_key: String::new(),
            source_language: "en".into(),
            target_language: "zh-CN".into(),
        }
    }
    fn remote_config() -> TranslationConfig {
        TranslationConfig {
            endpoint: "https://api.example.com/v1".into(),
            api_key: uuid::Uuid::new_v4().to_string(),
            ..local_config()
        }
    }

    #[test]
    fn restart_restores_default_profile_languages_and_vault_secret_without_plaintext_on_disk() {
        let harness = Harness::new();
        assert!(harness
            .service()
            .load()
            .expect("空配置")
            .endpoint
            .is_empty());
        let config = remote_config();
        harness.service().save(config.clone()).expect("保存配置");
        let restarted_database =
            Database::initialize(&harness.directory).expect("模拟重启打开数据库");
        let restarted = TranslationConfigService {
            database: Ok(restarted_database.clone()),
            credentials: harness.credentials.clone(),
            access: Mutex::new(()),
        };
        assert!(restarted.load().expect("恢复配置") == config);
        let snapshot = BootstrapService::new(Ok(restarted_database))
            .snapshot(vec![])
            .expect("恢复启动快照");
        assert_eq!(snapshot.settings.translation.source_language, "en");
        let model = snapshot.default_model.expect("默认模型");
        assert!(model.has_secret);
        assert_eq!(model.model_name, "test-model");
        let connection = harness.database.connect().expect("检查存储");
        let stored: (String, String) = connection
            .query_row(
                "SELECT secret_ref, options_json FROM sys_model_profiles",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("仅有引用");
        assert!(stored.0.starts_with("translation-"));
        assert!(!stored.1.contains(&config.api_key));
        connection
            .execute_batch("PRAGMA wal_checkpoint(TRUNCATE)")
            .expect("刷新磁盘");
        for file in std::fs::read_dir(&harness.directory).expect("扫描隔离测试文件") {
            let data = std::fs::read(file.expect("目录条目").path()).expect("读取隔离文件");
            assert!(!data
                .windows(config.api_key.len())
                .any(|bytes| bytes == config.api_key.as_bytes()));
        }
        assert!(!format!("{config:?}").contains(&config.api_key));
    }

    #[test]
    fn repeated_save_reuses_profile_and_reference_and_preserves_other_preferences() {
        let harness = Harness::new();
        let service = harness.service();
        let mut config = remote_config();
        service.save(config.clone()).expect("首次保存");
        let old_ref = translation_config::load(&harness.database)
            .expect("读取引用")
            .secret_ref;
        harness
            .database
            .connect()
            .expect("打开数据库")
            .execute_batch(
                "UPDATE sys_model_profiles SET options_json = '{\"temperature\":0.4}';
             UPDATE sys_translation_settings SET save_history = 0, auto_interval_ms = 2500;",
            )
            .expect("设置未编辑的选项");
        config.model = "changed-model".into();
        config.source_language = "ja".into();
        service.save(config).expect("再次保存");
        assert_eq!(
            translation_config::load(&harness.database)
                .expect("读取引用")
                .secret_ref,
            old_ref
        );
        let snapshot = BootstrapService::new(Ok(harness.database.clone()))
            .snapshot(vec![])
            .expect("快照");
        assert!(!snapshot.settings.translation.save_history);
        assert_eq!(snapshot.settings.translation.auto_interval_ms, 2500);
        assert_eq!(
            snapshot.default_model.expect("模型").options["temperature"],
            0.4
        );
        let count: i64 = harness
            .database
            .connect()
            .expect("数据库")
            .query_row("SELECT COUNT(*) FROM sys_model_profiles", [], |row| {
                row.get(0)
            })
            .expect("配置数");
        assert_eq!(count, 1);
        assert_eq!(harness.credentials.values.lock().expect("凭据数").len(), 1);
    }

    #[test]
    fn replacing_or_clearing_secret_removes_only_unreferenced_old_credentials() {
        let harness = Harness::new();
        let service = harness.service();
        service.save(remote_config()).expect("首次保存");
        let old_ref = translation_config::load(&harness.database)
            .expect("旧引用")
            .secret_ref
            .expect("有凭据");
        let replacement = remote_config();
        service.save(replacement.clone()).expect("替换密钥");
        assert!(harness
            .credentials
            .read(&old_ref)
            .expect("检查旧密钥")
            .is_none());
        assert!(service.load().expect("读取新配置") == replacement);
        service
            .save(local_config())
            .expect("切换无需密钥的本地服务");
        assert!(translation_config::load(&harness.database)
            .expect("无密钥引用")
            .secret_ref
            .is_none());
        assert!(harness
            .credentials
            .values
            .lock()
            .expect("凭据已清理")
            .is_empty());
    }

    #[test]
    fn vault_failure_and_missing_credential_have_recoverable_behavior() {
        let harness = Harness::new();
        let service = harness.service();
        let original = remote_config();
        service.save(original.clone()).expect("原配置");
        harness
            .credentials
            .fail_write
            .store(true, Ordering::Relaxed);
        assert_eq!(
            service.save(remote_config()).expect_err("凭据拒绝"),
            CaptureTranslationError::CredentialUnavailable
        );
        assert!(service.load().expect("旧配置仍可读取") == original);
        harness
            .credentials
            .fail_write
            .store(false, Ordering::Relaxed);
        harness.credentials.fail_read.store(true, Ordering::Relaxed);
        assert_eq!(
            service.load().expect_err("凭据拒绝读取"),
            CaptureTranslationError::CredentialUnavailable
        );
        assert!(service.save(remote_config()).is_err());
        harness
            .credentials
            .fail_read
            .store(false, Ordering::Relaxed);
        harness
            .credentials
            .values
            .lock()
            .expect("模拟外部删除")
            .clear();
        let missing = service.load().expect("密钥丢失仍可编辑");
        assert!(missing.api_key.is_empty());
        assert_eq!(missing.model, original.model);
        service.save(remote_config()).expect("重新填写密钥恢复");
        assert!(!service.load().expect("恢复密钥").api_key.is_empty());
    }

    #[test]
    fn failed_database_transaction_keeps_old_profile_and_secret_and_cleans_staged_secret() {
        let harness = Harness::new();
        let service = harness.service();
        let original = remote_config();
        service.save(original.clone()).expect("原配置");
        harness
            .database
            .connect()
            .expect("数据库")
            .execute_batch(
                "CREATE TRIGGER reject_settings BEFORE UPDATE ON sys_translation_settings
             BEGIN SELECT RAISE(ABORT, '测试写入失败'); END;",
            )
            .expect("模拟数据库写入失败");
        let replacement = TranslationConfig {
            model: "replacement-model".into(),
            source_language: "ja".into(),
            ..remote_config()
        };
        assert_eq!(
            service.save(replacement).expect_err("事务必须失败"),
            CaptureTranslationError::DatabaseUnavailable
        );
        assert!(service.load().expect("原配置完整保留") == original);
        assert_eq!(
            harness
                .credentials
                .values
                .lock()
                .expect("没有暂存凭据")
                .len(),
            1
        );
    }

    #[test]
    fn cleanup_failure_does_not_report_a_committed_save_as_failed() {
        let harness = Harness::new();
        let service = harness.service();
        service.save(remote_config()).expect("原配置");
        harness
            .credentials
            .fail_delete
            .store(true, Ordering::Relaxed);
        let replacement = remote_config();
        service
            .save(replacement.clone())
            .expect("清理失败不影响已提交配置");
        assert!(service.load().expect("读取新配置") == replacement);
    }

    #[test]
    fn unreadable_staged_secret_is_removed_before_any_database_change() {
        let harness = Harness::new();
        let service = harness.service();
        let original = local_config();
        service.save(original.clone()).expect("原本地配置");
        harness.credentials.fail_read.store(true, Ordering::Relaxed);
        assert_eq!(
            service.save(remote_config()).expect_err("新凭据无法验证"),
            CaptureTranslationError::CredentialUnavailable
        );
        assert!(harness
            .credentials
            .values
            .lock()
            .expect("暂存条目清理")
            .is_empty());
        harness
            .credentials
            .fail_read
            .store(false, Ordering::Relaxed);
        assert!(service.load().expect("未切换到新配置") == original);
    }

    #[test]
    fn shared_secret_reference_is_not_deleted_when_default_profile_changes() {
        let harness = Harness::new();
        let service = harness.service();
        service.save(remote_config()).expect("原配置");
        let reference = translation_config::load(&harness.database)
            .expect("原引用")
            .secret_ref
            .expect("密钥引用");
        harness.database.connect().expect("数据库").execute(
            "INSERT INTO sys_model_profiles(name, provider, model_name, secret_ref) VALUES ('其他模型', 'openai_compatible', 'other-model', ?1)",
            [&reference],
        ).expect("其他模型共享引用");
        service.save(remote_config()).expect("替换默认密钥");
        assert!(harness
            .credentials
            .read(&reference)
            .expect("共享条目保留")
            .is_some());
    }

    #[test]
    fn concurrent_saves_leave_one_consistent_profile_and_credential() {
        let harness = Harness::new();
        let service = Arc::new(harness.service());
        let configs: Vec<_> = (0..4)
            .map(|index| TranslationConfig {
                model: format!("test-model-{index}"),
                ..remote_config()
            })
            .collect();
        let workers: Vec<_> = configs
            .iter()
            .map(|config| {
                let config = config.clone();
                let service = Arc::clone(&service);
                std::thread::spawn(move || {
                    service.save(config).expect("并发保存");
                })
            })
            .collect();
        for worker in workers {
            worker.join().expect("保存线程结束");
        }
        let loaded = service.load().expect("并发后读取");
        assert!(configs.iter().any(|config| config == &loaded));
        assert_eq!(
            harness
                .credentials
                .values
                .lock()
                .expect("只保留当前凭据")
                .len(),
            1
        );
        let count: i64 = harness
            .database
            .connect()
            .expect("数据库")
            .query_row("SELECT COUNT(*) FROM sys_model_profiles", [], |row| {
                row.get(0)
            })
            .expect("模型数");
        assert_eq!(count, 1);
    }

    #[test]
    fn invalid_input_never_changes_persisted_configuration() {
        let harness = Harness::new();
        let service = harness.service();
        let original = local_config();
        service.save(original.clone()).expect("保存有效配置");
        for endpoint in [
            "file:///invalid",
            "https://",
            "https://api.example.com/v1",
            "https://user:password@api.example.com/v1",
            "http://192.168.1.2/v1",
            "http://127.0.0.1:11434/v1?token=value",
            "http://127.0.0.1:11434/v1#fragment",
        ] {
            assert_eq!(
                service
                    .save(TranslationConfig {
                        endpoint: endpoint.into(),
                        ..original.clone()
                    })
                    .expect_err("端点无效"),
                CaptureTranslationError::TranslationNotConfigured
            );
        }
        assert!(service
            .save(TranslationConfig {
                target_language: "auto".into(),
                ..original.clone()
            })
            .is_err());
        assert!(service
            .save(TranslationConfig {
                model: "a".repeat(257),
                ..original.clone()
            })
            .is_err());
        assert!(service.load().expect("原配置") == original);
        let normalized = service
            .save(TranslationConfig {
                endpoint: " http://[::1]:11434/v1 ".into(),
                model: " model ".into(),
                source_language: " en ".into(),
                ..original
            })
            .expect("IPv6 本地模型");
        assert_eq!(normalized.endpoint, "http://[::1]:11434/v1");
        assert_eq!(normalized.model, "model");
        assert_eq!(normalized.source_language, "en");
    }

    #[test]
    fn unavailable_database_exposes_only_safe_errors() {
        let service =
            TranslationConfigService::new(Err(DatabaseError::InvalidData("内部路径和诊断".into())));
        let error = service.load().expect_err("数据库不可用");
        assert_eq!(error, CaptureTranslationError::DatabaseUnavailable);
        let public = crate::domain::captured_layout::PublicError::from(error);
        assert_eq!(public.code, "database_error");
        assert!(!public.message.contains("内部路径"));
        assert!(service.save(local_config()).is_err());
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    #[test]
    #[ignore = "需要访问真实系统凭据存储；使用隔离数据库和临时条目"]
    fn system_vault_roundtrip_across_process_restart() {
        let harness = Harness::new();
        let service = TranslationConfigService::new(Ok(harness.database.clone()));
        let config = remote_config();
        service.save(config.clone()).expect("原生保存配置");
        struct Cleanup(String);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = SystemCredentialStore.delete(&self.0);
            }
        }
        let reference = translation_config::load(&harness.database)
            .expect("读取原生引用")
            .secret_ref
            .expect("存在引用");
        let cleanup = Cleanup(reference);
        assert!(service.load().expect("原生读取配置") == config);
        // 独立进程重新打开同一数据库和系统凭据，验证配置不依赖父进程内存。
        let status = std::process::Command::new(std::env::current_exe().expect("测试可执行文件"))
            .args([
                "services::translation_config::tests::native_child_restore",
                "--exact",
                "--ignored",
            ])
            .env("LORELINGO_TEST_SETTINGS_DIRECTORY", &harness.directory)
            .status()
            .expect("启动隔离恢复进程");
        assert!(status.success());
        SystemCredentialStore
            .delete(&cleanup.0)
            .expect("清理原生临时凭据");
        assert!(SystemCredentialStore
            .read(&cleanup.0)
            .expect("确认原生清理")
            .is_none());
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    #[test]
    #[ignore = "仅由原生跨进程测试调用"]
    fn native_child_restore() {
        let Some(directory) = std::env::var_os("LORELINGO_TEST_SETTINGS_DIRECTORY") else {
            return;
        };
        let database =
            Database::initialize(std::path::Path::new(&directory)).expect("子进程打开数据库");
        let config = TranslationConfigService::new(Ok(database))
            .load()
            .expect("子进程恢复原生配置");
        assert_eq!(config.endpoint, "https://api.example.com/v1");
        assert_eq!(config.model, "test-model");
        assert!(uuid::Uuid::parse_str(&config.api_key).is_ok());
    }
}
