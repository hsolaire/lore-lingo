# Testing and Quality Standards

## Risk-Based Coverage

- Add focused tests for every bug fix and stable contract change.
- Broaden coverage for shared services, IPC DTOs, migrations, platform adapters, multi-window state, and release configuration.
- Test failure and recovery paths, not only successful output.
- Keep tests deterministic; control time, external services, display metadata, and filesystem state through explicit seams.

## Required Checks

Current minimum for source changes:

```text
pnpm build
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
```

As test/lint scripts are added, the standard gate becomes:

```text
pnpm lint
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

Do not claim a command was run when the repository does not yet define it. Record missing test infrastructure as a concrete gap.

## Test Layers

- Unit: pure transforms, parsing, domain rules, error mapping, state machines.
- Component: UI states, keyboard behavior, event cleanup, stale-response handling.
- Integration: Tauri command/service/repository boundaries and SQLite migrations.
- Native smoke: permissions, windows, displays, DPI, overlay behavior, packaging.
- Release smoke: install, first launch, upgrade, restart after permissions, uninstall.

## Review Checklist

- Behavior matches the request and active specs.
- New types and errors are stable across IPC.
- Resources, listeners, workers, handles, and transactions are cleaned up.
- Platform-specific behavior has a fallback or explicit unsupported state.
- Sensitive data is absent from code, tests, logs, and snapshots.
- Documentation and tasks reflect material behavior changes.
- The working tree contains no accidental build or generated output.

