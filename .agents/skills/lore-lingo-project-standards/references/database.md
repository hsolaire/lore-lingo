# Database Standards

Use `docs/database-design.md` as the authoritative schema design and `src-tauri/migrations/` as executable history.

## Ownership

- Rust exclusively opens SQLite, executes migrations, and runs SQL.
- Vue calls narrow typed commands and MUST NOT construct SQL or reference table names.
- Keep runtime-only state such as active sessions and native window handles out of SQLite.

## Modules and Object Naming

Every application-owned table MUST use one of these domain prefixes:

| Prefix | Owner | Examples |
| --- | --- | --- |
| `db_` | SQLite lifecycle and internal database management | `db_schema_migrations` |
| `sys_` | Application-level models, preferences, and device configuration | `sys_model_profiles`, `sys_window_states` |
| `sub_` | Subtitle library, parsed entries, and subtitle search | `sub_files`, `sub_entries_fts` |
| `tr_` | Translation sessions, history, search, and disposable cache | `tr_history`, `tr_cache` |

- Treat prefixes as ownership boundaries, not SQLite schemas; the database remains one transaction boundary.
- Use lowercase `snake_case` and plural table names except established mass nouns such as `history` and `cache`.
- Keep FTS tables in the owning domain and suffix them with `_fts`.
- Name normal indexes `ix_<table>_<purpose>`, unique indexes `ux_<table>_<purpose>`, and triggers `trg_<table>_<purpose>_<event>`.
- Do not introduce another table prefix without updating this reference and `docs/database-design.md` in the same change.

## Schema Conventions

- Use `INTEGER PRIMARY KEY` for local surrogate identifiers. Add `AUTOINCREMENT` only when rowid reuse is a demonstrated correctness problem.
- Name foreign keys `<singular_entity>_id`, declare the referenced table explicitly, and choose `ON DELETE` behavior deliberately.
- Use `INTEGER` plus `CHECK (... IN (0, 1))` for booleans and use positive names such as `is_enabled` or `save_history`.
- Store timestamps as Unix milliseconds in `INTEGER` columns suffixed `_at`; suffix durations `_ms` and byte counts `_bytes`.
- Store enum-like values as lowercase `TEXT` with a `CHECK` constraint when the allowed set is owned by the application.
- Suffix JSON text columns `_json`, require `json_valid`, and constrain the top-level JSON type when it is known.
- Keep hashes explicit (`*_hash` or `fingerprint`), document the algorithm, and validate the expected encoded length when fixed.
- Prefer `STRICT` tables. Use `WITHOUT ROWID` for stable text or composite primary keys when it materially avoids a redundant rowid.
- Keep exactly one migration-version authority; do not independently advance both a custom history table and `PRAGMA user_version`.

## Schema Documentation

SQLite has no native `COMMENT ON TABLE/COLUMN` metadata.

- Put a concise table description immediately after `CREATE TABLE <name> (` and a concise column description on the defining line.
- Enumerate owned status/enum values as `value=meaning`, state units for numeric time/size fields, and describe the top-level shape of JSON columns.
- Treat inline DDL comments as inspection help, not structured metadata; keep `docs/database-design.md` authoritative.
- Do not add a custom comments table unless a concrete runtime feature needs queryable schema descriptions.

## Migrations

- Treat every committed/released migration as append-only and immutable.
- Add schema changes in the next zero-padded migration file; never rewrite `0001_initial.sql` after it is adopted.
- Wrap related DDL/data changes in an explicit transaction and update migration metadata consistently.
- Make migrations deterministic, ordered, and safe to run exactly once.
- Test both a fresh database and an upgrade from the previous schema version.
- Include rollback/recovery notes for destructive or long-running transformations even when SQLite cannot perform an automatic down migration.

## Schema and Queries

- Use typed columns and constraints for fields that are validated, related, filtered, or sorted.
- Use JSON only for low-frequency preferences or provider-specific options and enforce `json_valid`.
- Enable foreign keys, WAL mode, and a bounded busy timeout for each connection.
- Use transactions for multi-table writes and parameterized queries for every external value.
- Add indexes from demonstrated query patterns; inspect query plans for history, subtitle, and cache hot paths.
- Keep history and cache lifecycles separate.

## Secrets and Privacy

- Store only keychain/credential-manager references in SQLite; never store API keys or auth headers.
- Treat database files, backups, exports, and diagnostics as user-sensitive data.
- Do not log source/translated text by default when it is not required for diagnostics.

## Repository Boundary

- Prefer repositories or focused database services over a generic key-value or `execute_sql` IPC API.
- Map rows to domain types inside Rust and return stable DTOs to Vue.
- Keep transaction scope at the use-case level when one action updates multiple repositories.
