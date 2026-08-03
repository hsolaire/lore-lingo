# Release Standards

## Supported Targets

- Windows 10 22H2 and Windows 11, x86_64 MSVC, with a documented WebView2 strategy.
- macOS 13 or newer, Universal 2, signed and notarized for direct distribution.
- Build each operating-system artifact on a native CI runner; do not claim host-local cross-platform bundles.

## Pipeline Stages

1. Validate lockfiles, versions, formatting, types, tests, and OpenSpec artifacts.
2. Build unsigned native artifacts in pull-request CI.
3. Run platform smoke tests for permissions, capture, scaling, multiple displays, and overlays.
4. Build release artifacts from an authorized SemVer tag.
5. Sign Windows artifacts and sign/notarize macOS artifacts using protected CI secrets.
6. Verify installer launch and publish checksums and release notes.

## Versioning

- Keep `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` versions synchronized.
- Use SemVer based on user-visible behavior and compatibility impact.
- Record schema migrations, minimum operating-system changes, and breaking configuration changes in release notes.

## Packaging

- Never commit `dist/`, `target/`, installers, signing material, or notarization credentials.
- Verify application identifier, icons, usage descriptions, entitlements, capabilities, and installer metadata before release.
- Keep signing optional for untrusted pull requests and mandatory for publication jobs.

## Release Gate

- Required CI is green on Windows and macOS.
- Database fresh-install and upgrade migrations pass.
- Compatibility smoke matrix is complete for the release candidate.
- No known critical/high security finding remains unresolved.
- Rollback or recovery instructions exist for migrations and distribution failures.

