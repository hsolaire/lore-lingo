# Directory Structure

## Purpose

Keep ownership boundaries visible from paths. New files MUST follow this target structure; existing files MAY migrate incrementally when touched by a coherent change.

## Repository Layout

```text
lore-lingo/
  .agents/skills/               Project-scoped Codex Skills
  .github/workflows/            CI and release automation
  docs/                         Durable architecture and operational docs
  openspec/                     Capability specs and change artifacts
  prototype/                    Non-production visual prototypes only
  src/                          Vue/TypeScript frontend
  src-tauri/                    Rust/Tauri desktop backend
  package.json                  Frontend scripts and dependencies
  pnpm-lock.yaml                Sole JavaScript lockfile
```

## Frontend Layout

```text
src/
  components/
    ui/                         Reusable visual primitives
    main-panel/                 Main-window feature components
    translation-overlay/       Target location for translation-overlay components
    lyric/                      Legacy path; migrate coherently and add no new files
    settings/                   Settings feature components
  composables/                  Reusable reactive orchestration
  services/                     Typed Tauri/API clients; add when needed
  types/                        Shared frontend contract types; add when needed
  icons/                        Icon registry
  styles/                       Global tokens and cross-window styles
  App.vue                       Main window composition root
  OverlayApp.vue                Translation-overlay composition root
  main.ts                       Main entry
  overlay.ts                    Overlay entry
```

Rules:

- Components own rendering and interaction, not native integration or persistence.
- Composables own Vue state and lifecycle orchestration, not raw platform details.
- All direct Tauri `invoke` calls SHOULD move behind typed modules in `src/services/` as the codebase grows.
- Shared wire types belong in `src/types/`; do not redefine the same command payload in multiple components.
- `prototype/` MUST NOT be imported by production entries.

## Backend Layout

```text
src-tauri/
  capabilities/                 Tauri capability declarations
  migrations/                   Append-only SQLite migrations
  src/
    commands/                   Thin IPC handlers; add by domain
    db/                         Connection, migrations, repositories
    desktop/                    Desktop capability facade and capture sessions
      platform/                 Target-gated Windows/macOS adapters
    domain/                     Stable business types and rules
    services/                   Use-case orchestration
    lib.rs                      Application composition and registration
    main.rs                     Binary entry only
  Cargo.toml
  tauri.conf.json
```

Rules:

- `lib.rs` and command modules MUST NOT accumulate domain algorithms.
- Target-specific dependencies and unsafe code MUST remain under the relevant platform module.
- Database queries MUST remain under `db/` repositories or focused database services.
- Commands MUST expose use cases, never generic SQL, table names, filesystem paths, or native handles.

## Naming

- Vue components: `PascalCase.vue`.
- TypeScript modules and composables: existing camel-case convention, composables prefixed with `use`.
- Rust modules/files: `snake_case`.
- OpenSpec capabilities and Skill folders: `kebab-case`.
- New translation-overlay identifiers: `translationOverlay` in TypeScript and `translation-overlay` in paths/events; do not add new `lyric` identifiers.
- SQLite migrations: zero-padded sequence plus snake-case description, for example `0002_add_capture_profiles.sql`.
- Tests: mirror the source ownership boundary and use the framework-standard test suffix.

## Placement Decision

Before creating a file, identify its owner and consumers. If a file serves multiple unrelated domains, split the responsibility instead of creating a generic `utils` dumping ground.
