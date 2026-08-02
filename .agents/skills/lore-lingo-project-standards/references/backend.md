# Backend Standards

## Layering

```text
Tauri command -> service/use case -> domain/repository/platform adapter
```

- Commands validate and translate IPC data, call one use case, and return stable DTOs/errors.
- Services coordinate capture, persistence, OCR, translation, and events.
- Domain modules contain platform-neutral business rules and types.
- Platform adapters contain Windows/macOS frameworks, handles, permissions, and unsafe code.
- Repositories own SQLite queries and transaction boundaries.

## Rust Rules

- Use typed structs/enums instead of stringly typed internal state.
- Return `Result` for fallible operations; do not panic on user data, permissions, I/O, or unavailable windows.
- Convert internal errors to stable public error codes at the command boundary.
- Avoid blocking the Tauri UI thread. Isolate blocking database/native calls or use a dedicated worker.
- Bound queues and caches. Capture and OCR pipelines MUST define backpressure and cancellation.
- Release native resources through RAII where possible; keep each `unsafe` block minimal and explain its invariant.
- Gate operating-system dependencies and modules with target-specific Cargo configuration.

## Tauri Rules

- Grant the minimum capabilities needed by each window.
- Register commands by domain and keep `lib.rs` focused on application composition.
- Do not expose generic shell execution, arbitrary filesystem access, raw SQL, or native handles to WebViews.
- Treat events as notifications; keep authoritative state in managed Rust services or durable storage.
- Return safe user-facing errors and keep detailed native diagnostics out of production IPC payloads.

## Concurrency and Sessions

- Give every long-running operation an owner, cancellation path, terminal state, and cleanup path.
- Support replacement semantics explicitly when only one session may run.
- Prefer latest-value semantics for screen frames over unbounded FIFO queues.
- Never hold a global mutex while performing slow network, encoding, or native capture work.

## Validation

- Run `cargo fmt --check --manifest-path src-tauri/Cargo.toml`.
- Run `cargo check --manifest-path src-tauri/Cargo.toml` on the current platform.
- Run focused `cargo test` and `cargo clippy` as those suites become available.
- Compile target-gated code on native Windows and macOS CI; a macOS-only check cannot validate Windows modules.

