use std::collections::{BTreeMap, HashSet};

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};

use super::DatabaseError;

#[derive(Clone, Copy, Debug)]
pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "initial",
    sql: include_str!("../../migrations/0001_initial.sql"),
}];

pub fn run_migrations(connection: &mut Connection) -> Result<(), DatabaseError> {
    run_migration_set(connection, MIGRATIONS)
}

pub fn run_migration_set(
    connection: &mut Connection,
    migrations: &[Migration],
) -> Result<(), DatabaseError> {
    validate_migration_definitions(migrations)?;
    let known = migrations
        .iter()
        .map(|migration| (migration.version, migration.name))
        .collect::<BTreeMap<_, _>>();
    let applied = load_applied_migrations(connection)?;
    validate_applied_history(&known, &applied)?;

    // 步骤 1. 按版本顺序筛选尚未执行的迁移
    let mut pending = migrations
        .iter()
        .filter(|migration| !applied.contains_key(&migration.version))
        .collect::<Vec<_>>();
    pending.sort_by_key(|migration| migration.version);

    // 步骤 2. 每个迁移在独立立即事务中执行并登记历史
    for migration in pending {
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let body = migration_body(migration.sql)?;
        transaction.execute_batch(&body)?;
        let recorded_name = transaction
            .query_row(
                "SELECT name FROM db_schema_migrations WHERE version = ?1",
                [migration.version],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        match recorded_name {
            Some(name) if name == migration.name => {}
            Some(name) => {
                return Err(DatabaseError::Migration(format!(
                    "版本 {} 已登记为 {name}，预期为 {}",
                    migration.version, migration.name
                )));
            }
            None => {
                transaction.execute(
                    "INSERT INTO db_schema_migrations(version, name) VALUES (?1, ?2)",
                    params![migration.version, migration.name],
                )?;
            }
        }
        transaction.commit()?;
    }

    // 步骤 3. 重新读取历史，确保最终状态与内置迁移完全一致
    let applied = load_applied_migrations(connection)?;
    validate_applied_history(&known, &applied)?;
    Ok(())
}

fn validate_migration_definitions(migrations: &[Migration]) -> Result<(), DatabaseError> {
    let mut versions = HashSet::new();
    let mut names = HashSet::new();
    for migration in migrations {
        if migration.version <= 0 || migration.name.trim().is_empty() {
            return Err(DatabaseError::Migration("迁移版本和名称不能为空".into()));
        }
        if !versions.insert(migration.version) {
            return Err(DatabaseError::Migration(format!(
                "迁移版本 {} 重复",
                migration.version
            )));
        }
        if !names.insert(migration.name) {
            return Err(DatabaseError::Migration(format!(
                "迁移名称 {} 重复",
                migration.name
            )));
        }
    }
    Ok(())
}

fn load_applied_migrations(
    connection: &Connection,
) -> Result<BTreeMap<i64, String>, DatabaseError> {
    let exists: i64 = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'db_schema_migrations')",
        [],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Ok(BTreeMap::new());
    }

    let mut statement =
        connection.prepare("SELECT version, name FROM db_schema_migrations ORDER BY version")?;
    let rows = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
    let mut applied = BTreeMap::new();
    for row in rows {
        let (version, name) = row?;
        applied.insert(version, name);
    }
    Ok(applied)
}

fn validate_applied_history(
    known: &BTreeMap<i64, &'static str>,
    applied: &BTreeMap<i64, String>,
) -> Result<(), DatabaseError> {
    for (version, name) in applied {
        match known.get(version) {
            Some(expected) if *expected == name => {}
            Some(expected) => {
                return Err(DatabaseError::Migration(format!(
                    "历史版本 {version} 的名称为 {name}，预期为 {expected}"
                )));
            }
            None => {
                return Err(DatabaseError::Migration(format!(
                    "数据库包含应用未知的更高迁移版本 {version}"
                )));
            }
        }
    }

    if let Some(highest) = applied.keys().next_back() {
        for version in known.keys().filter(|version| *version <= highest) {
            if !applied.contains_key(version) {
                return Err(DatabaseError::Migration(format!(
                    "迁移历史在版本 {version} 处存在缺口"
                )));
            }
        }
    }
    Ok(())
}

fn migration_body(sql: &str) -> Result<String, DatabaseError> {
    let without_begin = sql.replacen("BEGIN IMMEDIATE;", "", 1);
    let trimmed = without_begin.trim_end();
    let Some(body) = trimmed.strip_suffix("COMMIT;") else {
        return Err(DatabaseError::Migration(
            "迁移必须以 COMMIT; 结束，便于 runner 接管事务".into(),
        ));
    };
    Ok(body.to_string())
}
