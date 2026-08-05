use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use lore_lingo_lib::db::{
    connection::Database,
    migrations::{run_migration_set, Migration, MIGRATIONS},
};
use rusqlite::Connection;

fn test_path(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("系统时间")
        .as_nanos();
    std::env::temp_dir().join(format!("lore-lingo-{name}-{nonce}.sqlite3"))
}

fn cleanup(path: &PathBuf) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(format!("{}-wal", path.display()));
    let _ = fs::remove_file(format!("{}-shm", path.display()));
}

#[test]
fn fresh_database_runs_initial_migration_once() {
    let path = test_path("fresh");
    let database = Database::initialize_file(path.clone()).expect("首次初始化数据库");
    let connection = database.connect().expect("打开数据库");
    let count: i64 = connection
        .query_row("SELECT COUNT(*) FROM db_schema_migrations", [], |row| {
            row.get(0)
        })
        .expect("读取迁移历史");
    assert_eq!(count, 1);
    drop(connection);
    drop(database);

    let database = Database::initialize_file(path.clone()).expect("重复初始化数据库");
    let connection = database.connect().expect("再次打开数据库");
    let count: i64 = connection
        .query_row("SELECT COUNT(*) FROM db_schema_migrations", [], |row| {
            row.get(0)
        })
        .expect("再次读取迁移历史");
    assert_eq!(count, 1);
    drop(connection);
    drop(database);
    cleanup(&path);
}

#[test]
fn failed_migration_rolls_back_schema_and_history() {
    const BAD_MIGRATION: Migration = Migration {
        version: 2,
        name: "broken",
        sql: "BEGIN IMMEDIATE;
              CREATE TABLE should_rollback(id INTEGER PRIMARY KEY);
              INSERT INTO missing_table(id) VALUES (1);
              COMMIT;",
    };

    let path = test_path("rollback");
    let database = Database::initialize_file(path.clone()).expect("初始化数据库");
    let mut connection = database.connect().expect("打开数据库");
    let migrations = [MIGRATIONS[0], BAD_MIGRATION];
    assert!(run_migration_set(&mut connection, &migrations).is_err());
    let table_exists: i64 = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name = 'should_rollback')",
            [],
            |row| row.get(0),
        )
        .expect("检查回滚表");
    let version_exists: i64 = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM db_schema_migrations WHERE version = 2)",
            [],
            |row| row.get(0),
        )
        .expect("检查迁移历史");
    assert_eq!((table_exists, version_exists), (0, 0));
    drop(connection);
    drop(database);
    cleanup(&path);
}

#[test]
fn connections_enable_foreign_keys_wal_json_and_trigram_fts() {
    let path = test_path("capabilities");
    let database = Database::initialize_file(path.clone()).expect("初始化数据库");
    let connection = database.connect().expect("打开数据库");
    let foreign_keys: i64 = connection
        .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
        .expect("读取外键配置");
    let journal_mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .expect("读取日志模式");
    let json_valid: i64 = connection
        .query_row("SELECT json_valid('{\"ok\":true}')", [], |row| row.get(0))
        .expect("验证 JSON");
    assert_eq!(foreign_keys, 1);
    assert_eq!(journal_mode, "wal");
    assert_eq!(json_valid, 1);

    connection
        .execute(
            "INSERT INTO sub_files(path, file_name, format, fingerprint, file_size_bytes, index_status)
             VALUES ('/tmp/demo.srt', 'demo.srt', 'srt', ?1, 10, 'ready')",
            ["a".repeat(64)],
        )
        .expect("插入字幕文件");
    connection
        .execute(
            "INSERT INTO sub_entries(subtitle_file_id, sequence_no, source_text)
             VALUES (1, 1, 'hello world')",
            [],
        )
        .expect("插入字幕条目");
    let fts_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sub_entries_fts WHERE sub_entries_fts MATCH 'ell'",
            [],
            |row| row.get(0),
        )
        .expect("验证 trigram 全文索引");
    assert_eq!(fts_count, 1);
    drop(connection);
    drop(database);
    cleanup(&path);
}

#[test]
fn constraints_reject_invalid_json_enum_and_shortcut_conflicts() {
    let path = test_path("constraints");
    let database = Database::initialize_file(path.clone()).expect("初始化数据库");
    let connection = database.connect().expect("打开数据库");
    assert!(connection
        .execute(
            "INSERT INTO sys_app_preferences(key, value_json) VALUES ('bad', '{')",
            [],
        )
        .is_err());
    assert!(connection
        .execute(
            "UPDATE sys_capture_settings SET mode = 'desktop' WHERE id = 1",
            []
        )
        .is_err());
    assert!(connection
        .execute(
            "INSERT INTO sys_shortcut_bindings(action, input_kind, binding_key, binding_json)
             VALUES ('other_action', 'keyboard', 'CommandOrControl+Shift+KeyT',
                     '{\"modifiers\":[\"CommandOrControl\",\"Shift\"],\"code\":\"KeyT\"}')",
            [],
        )
        .is_err());
    drop(connection);
    drop(database);
    cleanup(&path);
}

#[test]
fn migration_history_rejects_unknown_higher_version() {
    let path = test_path("unknown-version");
    let database = Database::initialize_file(path.clone()).expect("初始化数据库");
    let mut connection = database.connect().expect("打开数据库");
    connection
        .execute(
            "INSERT INTO db_schema_migrations(version, name) VALUES (99, 'future')",
            [],
        )
        .expect("构造更高版本历史");
    assert!(run_migration_set(&mut connection, MIGRATIONS).is_err());
    drop(connection);
    drop(database);
    cleanup(&path);
}

#[test]
fn hotspot_queries_use_existing_indexes() {
    let path = test_path("query-plan");
    let database = Database::initialize_file(path.clone()).expect("初始化数据库");
    let connection = database.connect().expect("打开数据库");
    let plans = [
        "EXPLAIN QUERY PLAN SELECT * FROM sub_entries WHERE subtitle_file_id = 1 ORDER BY sequence_no",
        "EXPLAIN QUERY PLAN SELECT * FROM tr_history ORDER BY created_at DESC LIMIT 20",
        "EXPLAIN QUERY PLAN SELECT * FROM tr_cache WHERE expires_at < 1",
    ];
    for sql in plans {
        let mut statement = connection.prepare(sql).expect("准备查询计划");
        let details = statement
            .query_map([], |row| row.get::<_, String>(3))
            .expect("读取查询计划")
            .collect::<Result<Vec<_>, _>>()
            .expect("收集查询计划")
            .join(" ");
        assert!(details.contains("INDEX"), "查询未使用索引：{details}");
    }
    drop(connection);
    drop(database);
    cleanup(&path);
}

#[test]
fn duplicate_migration_versions_are_rejected_before_execution() {
    const DUPLICATE: Migration = Migration {
        version: 1,
        name: "duplicate",
        sql: "BEGIN IMMEDIATE; SELECT 1; COMMIT;",
    };
    let mut connection = Connection::open_in_memory().expect("打开内存数据库");
    assert!(run_migration_set(&mut connection, &[MIGRATIONS[0], DUPLICATE]).is_err());
}
