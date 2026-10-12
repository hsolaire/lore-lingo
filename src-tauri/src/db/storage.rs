//! 集中配置目录和旧应用数据库的一次性、安全迁入。

use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use rusqlite::{
    backup::{Backup, StepResult},
    Connection, OpenFlags,
};

use super::{
    connection::{configure_connection, prepare_database, Database, DATABASE_FILE_NAME},
    DatabaseError,
};

const CONFIG_DIRECTORY_NAME: &str = ".lore-lingo";
const BACKUP_TIMEOUT: Duration = Duration::from_secs(5);

pub fn initialize_default(
    home: &Path,
    legacy_directory: Option<&Path>,
) -> Result<Database, DatabaseError> {
    if !home.is_absolute() {
        return Err(DatabaseError::InvalidData("用户主目录不可用".into()));
    }
    // 步骤 1. 主目录下只使用一个配置根目录，已有目标库始终优先。
    let directory = home.join(CONFIG_DIRECTORY_NAME);
    create_private_directory(&directory)?;
    let destination = directory.join(DATABASE_FILE_NAME);
    if destination.try_exists()? {
        return Database::initialize_file(destination);
    }
    // 步骤 2. 只在目标不存在时读取旧库；不创建、不移动或删除旧库。
    if let Some(legacy_directory) = legacy_directory {
        let source = legacy_directory.join(DATABASE_FILE_NAME);
        if source != destination && source.try_exists()? {
            migrate_database(&source, &destination)?;
        }
    }
    // 步骤 3. 迁入后或首次启动时正常执行已有模式初始化，不回退到其他目录。
    create_private_file(&destination, false)?;
    Database::initialize_file(destination)
}

fn create_private_directory(path: &Path) -> io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(path)
}

fn create_private_file(path: &Path, exclusive: bool) -> io::Result<fs::File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    if exclusive {
        options.create_new(true);
    } else {
        options.create(true).truncate(false);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)
}

fn migrate_database(source: &Path, destination: &Path) -> Result<(), DatabaseError> {
    // 步骤 1. 在目标目录独占创建临时文件，只读打开旧库，包含尚未检查点化的 WAL 数据。
    let parent = destination
        .parent()
        .ok_or_else(|| DatabaseError::InvalidData("配置目录不可用".into()))?;
    let staged_path = parent.join(format!(".migration-{}.sqlite3", uuid::Uuid::new_v4()));
    create_private_file(&staged_path, true)?;
    let staged = StagedDatabase(staged_path);
    let source_connection = Connection::open_with_flags(source, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    source_connection.busy_timeout(Duration::from_millis(100))?;
    let mut target_connection = Connection::open(&staged.0)?;
    {
        let backup = Backup::new(&source_connection, &mut target_connection)?;
        let deadline = Instant::now() + BACKUP_TIMEOUT;
        loop {
            match backup.step(128)? {
                StepResult::Done => break,
                StepResult::More | StepResult::Busy | StepResult::Locked => {
                    if Instant::now() >= deadline {
                        return Err(DatabaseError::Migration(
                            "旧配置迁入超时，请关闭旧应用后重试".into(),
                        ));
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                _ => return Err(DatabaseError::Migration("旧配置迁入失败".into())),
            }
        }
    }
    // 步骤 2. 验证快照和模式，所有迁移及默认值写入只在临时库中完成。
    let integrity: String =
        target_connection.query_row("PRAGMA quick_check", [], |row| row.get(0))?;
    if integrity != "ok" {
        return Err(DatabaseError::InvalidData("旧配置数据库校验失败".into()));
    }
    configure_connection(&target_connection)?;
    prepare_database(&mut target_connection)?;
    let mode: String =
        target_connection.query_row("PRAGMA journal_mode = DELETE", [], |row| row.get(0))?;
    if mode != "delete" {
        return Err(DatabaseError::Migration("临时配置尚未完成写入".into()));
    }
    target_connection
        .close()
        .map_err(|(_, error)| DatabaseError::from(error))?;
    create_private_file(&staged.0, false)?.sync_all()?;
    // 步骤 3. 同目录硬链接发布完整快照，原子拒绝覆盖并发创建的目标，保留旧库。
    match fs::hard_link(&staged.0, destination) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(error.into()),
    }
}

struct StagedDatabase(PathBuf);

impl Drop for StagedDatabase {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
        for suffix in ["-wal", "-shm", "-journal"] {
            let mut sidecar = self.0.as_os_str().to_os_string();
            sidecar.push(suffix);
            let _ = fs::remove_file(PathBuf::from(sidecar));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Workspace(PathBuf);
    impl Workspace {
        fn new() -> Self {
            let path =
                std::env::temp_dir().join(format!("lore-lingo-storage-{}", uuid::Uuid::new_v4()));
            fs::create_dir_all(&path).expect("创建隔离目录");
            Self(path)
        }
        fn home(&self) -> PathBuf {
            self.0.join("home")
        }
        fn legacy(&self) -> PathBuf {
            self.0.join("legacy")
        }
    }
    impl Drop for Workspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn source_language(database: &Database) -> String {
        database
            .connect()
            .expect("数据库连接")
            .query_row(
                "SELECT source_language FROM sys_translation_settings WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .expect("读取语言")
    }

    #[test]
    fn fresh_start_uses_a_single_home_directory_and_restarts_at_the_same_path() {
        let workspace = Workspace::new();
        let database =
            initialize_default(&workspace.home(), Some(&workspace.legacy())).expect("首次启动");
        assert_eq!(
            database.path(),
            workspace.home().join(".lore-lingo/lore-lingo.sqlite3")
        );
        assert_eq!(source_language(&database), "en");
        assert!(!workspace.legacy().exists());
        let restarted = initialize_default(&workspace.home(), None).expect("再次启动");
        assert_eq!(restarted.path(), database.path());
    }

    #[test]
    fn migration_includes_live_wal_preserves_secret_references_and_retains_original_database() {
        let workspace = Workspace::new();
        let legacy = Database::initialize(&workspace.legacy()).expect("建立旧库");
        let original = fs::read(legacy.path()).expect("旧库原始文件");
        let writer = legacy.connect().expect("保持旧写入连接");
        writer
            .execute_batch("PRAGMA wal_autocheckpoint = 0;")
            .expect("保留 WAL");
        writer.execute_batch(
            "INSERT INTO sys_model_profiles(name, provider, model_name, api_base, secret_ref)
             VALUES ('默认模型', 'openai_compatible', 'preserved-model', 'https://api.example.com/v1', 'translation-test-reference');
             UPDATE sys_translation_settings SET source_language = 'ja', default_model_profile_id = last_insert_rowid();",
        ).expect("旧库写入 WAL 配置");
        assert!(PathBuf::from(format!("{}-wal", legacy.path().display())).exists());
        let migrated =
            initialize_default(&workspace.home(), Some(&workspace.legacy())).expect("迁入旧配置");
        assert_eq!(source_language(&migrated), "ja");
        let profile: (String, String) = migrated
            .connect()
            .expect("目标连接")
            .query_row(
                "SELECT model_name, secret_ref FROM sys_model_profiles",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("保留模型和密钥引用");
        assert_eq!(
            profile,
            (
                "preserved-model".into(),
                "translation-test-reference".into()
            )
        );
        assert_eq!(fs::read(legacy.path()).expect("仍有旧库"), original);
        assert_eq!(source_language(&legacy), "ja");
        assert!(!fs::read_dir(workspace.home().join(CONFIG_DIRECTORY_NAME))
            .expect("目标目录")
            .any(|entry| {
                entry
                    .expect("条目")
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".migration-")
            }));
        let restarted =
            initialize_default(&workspace.home(), Some(&workspace.legacy())).expect("重复启动");
        assert_eq!(source_language(&restarted), "ja");
    }

    #[test]
    fn existing_target_is_authoritative_even_when_old_database_is_different_or_corrupt() {
        let workspace = Workspace::new();
        let target = initialize_default(&workspace.home(), None).expect("新目录数据库");
        target
            .connect()
            .expect("目标连接")
            .execute(
                "UPDATE sys_translation_settings SET source_language = 'ko' WHERE id = 1",
                [],
            )
            .expect("新配置");
        fs::create_dir_all(workspace.legacy()).expect("旧目录");
        fs::write(
            workspace.legacy().join(DATABASE_FILE_NAME),
            b"invalid SQLite",
        )
        .expect("无效旧库");
        let restored = initialize_default(&workspace.home(), Some(&workspace.legacy()))
            .expect("只读取现有目标");
        assert_eq!(source_language(&restored), "ko");
    }

    #[test]
    fn failed_migration_never_publishes_an_empty_database_and_can_be_retried() {
        let workspace = Workspace::new();
        fs::create_dir_all(workspace.legacy()).expect("旧目录");
        let source = workspace.legacy().join(DATABASE_FILE_NAME);
        fs::write(&source, b"invalid SQLite").expect("无效旧库");
        assert!(initialize_default(&workspace.home(), Some(&workspace.legacy())).is_err());
        assert_eq!(fs::read(&source).expect("旧文件保留"), b"invalid SQLite");
        assert_eq!(
            fs::read_dir(workspace.home().join(CONFIG_DIRECTORY_NAME))
                .expect("失败目录")
                .count(),
            0
        );
        fs::remove_file(&source).expect("修复隔离测试源");
        Database::initialize(&workspace.legacy()).expect("恢复有效旧库");
        assert!(initialize_default(&workspace.home(), Some(&workspace.legacy())).is_ok());
    }

    #[test]
    fn incompatible_schema_does_not_publish_or_modify_the_old_database() {
        let workspace = Workspace::new();
        let legacy = Database::initialize(&workspace.legacy()).expect("旧库");
        legacy
            .connect()
            .expect("连接")
            .execute(
                "INSERT INTO db_schema_migrations(version, name) VALUES (999, 'future')",
                [],
            )
            .expect("模拟新版本数据库");
        assert!(initialize_default(&workspace.home(), Some(&workspace.legacy())).is_err());
        assert_eq!(
            fs::read_dir(workspace.home().join(CONFIG_DIRECTORY_NAME))
                .expect("失败目录")
                .count(),
            0
        );
        let revision: i64 = legacy
            .connect()
            .expect("旧库可用")
            .query_row("SELECT MAX(version) FROM db_schema_migrations", [], |row| {
                row.get(0)
            })
            .expect("未修改旧版本");
        assert_eq!(revision, 999);
    }

    #[test]
    fn missing_home_or_unwritable_destination_does_not_fall_back_to_legacy_storage() {
        assert!(initialize_default(Path::new(""), None).is_err());
        assert!(initialize_default(Path::new("relative"), None).is_err());
        let workspace = Workspace::new();
        fs::create_dir_all(workspace.home()).expect("主目录");
        fs::write(workspace.home().join(CONFIG_DIRECTORY_NAME), b"blocked").expect("阻止建目录");
        let legacy = Database::initialize(&workspace.legacy()).expect("旧库");
        assert!(initialize_default(&workspace.home(), Some(&workspace.legacy())).is_err());
        assert_eq!(source_language(&legacy), "en");
    }

    #[cfg(unix)]
    #[test]
    fn new_configuration_files_have_private_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let workspace = Workspace::new();
        let database = initialize_default(&workspace.home(), None).expect("初始化私有目录");
        assert_eq!(
            fs::metadata(database.path().parent().expect("配置目录"))
                .expect("目录属性")
                .permissions()
                .mode()
                & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(database.path())
                .expect("数据库属性")
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
