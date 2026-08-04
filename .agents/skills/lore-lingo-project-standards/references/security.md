# Security Standards

## Secrets

- Store provider credentials in macOS Keychain or Windows Credential Manager through a Rust abstraction.
- Persist only secret references in SQLite and non-secret endpoints/options in configuration.
- Never place real-looking keys in defaults, prototypes, tests, screenshots, logs, commits, or error messages.
- Redact authorization headers, query credentials, local paths, captured text, and database content from diagnostics.

## Tauri and IPC

- Use least-privilege capability files per window.
- Expose narrow allowlisted commands with typed validated inputs.
- Do not expose arbitrary shell commands, SQL, file reads, URLs, or native handles.
- Validate lengths, paths, URL schemes, numeric ranges, source IDs, catalog revisions, and enum values in Rust.
- Keep the content security policy explicit for production; document any temporary relaxation.

## External Data

- Treat OCR output, subtitle files, model responses, filenames, window titles, and API errors as untrusted input.
- Do not render untrusted HTML; escape output and avoid dynamic code evaluation.
- Bound file sizes, frame sizes, queue lengths, response sizes, retries, and timeouts.
- Apply SSRF controls when custom API endpoints are supported; restrict unsafe schemes and local targets according to the product policy.

## Dependencies

- Prefer maintained dependencies with a narrow feature set and target-gate native packages.
- Review lockfile changes and transitive dependency growth.
- Run available Rust and JavaScript advisory checks in release CI.
- Document native framework minimum versions and fallback behavior.

## Privacy

- Capture only after explicit user action and visible state.
- Request macOS recording permission only when required and explain recovery from denial.
- Stop capture workers promptly and avoid retaining raw frames longer than the active pipeline requires.

