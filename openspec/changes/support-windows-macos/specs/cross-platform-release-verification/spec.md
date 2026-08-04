## ADDED Requirements

### Requirement: Reproducible project toolchains
The repository SHALL define one JavaScript package manager and a pinned Rust toolchain suitable for every supported build target.

#### Scenario: Developer installs frontend dependencies
- **WHEN** dependencies are installed on Windows or macOS using the documented command
- **THEN** the same committed pnpm lockfile controls dependency resolution and no second lockfile is required

#### Scenario: CI installs Rust
- **WHEN** a native CI build starts
- **THEN** it installs the pinned toolchain, required components, and declared target architecture

### Requirement: Native pull-request build matrix
Pull-request CI SHALL build and test the application on native Windows and macOS runners.

#### Scenario: Windows job runs
- **WHEN** a pull request changes desktop, frontend, or build files
- **THEN** CI runs frontend checks, Rust tests, and an unsigned Windows Tauri build on the supported x86_64 target

#### Scenario: macOS job runs
- **WHEN** a pull request changes desktop, frontend, or build files
- **THEN** CI runs frontend checks, Rust tests, and unsigned macOS builds covering Apple Silicon and Intel compatibility

#### Scenario: Platform-specific code does not compile
- **WHEN** either native platform job fails to compile its target-gated adapter
- **THEN** the pull-request build fails even if the other platform succeeds

### Requirement: Signed release artifacts
Release automation SHALL separate unsigned verification from credentialed signing and SHALL produce installable artifacts for supported platforms.

#### Scenario: Windows release tag is built
- **WHEN** an authorized release tag triggers the Windows job
- **THEN** it produces signed x86_64 installer artifacts and applies the documented WebView2 installation strategy

#### Scenario: macOS release tag is built
- **WHEN** an authorized release tag triggers the macOS job
- **THEN** it produces signed and notarized artifacts covering Apple Silicon and Intel targets

#### Scenario: Signing credentials are unavailable
- **WHEN** signing secrets are absent in a pull request or untrusted context
- **THEN** CI performs unsigned verification without printing, requesting, or fabricating credentials

### Requirement: Compatibility smoke-test matrix
Every release candidate MUST be checked against a documented Windows/macOS workflow matrix before publication.

#### Scenario: Display and scaling workflows are tested
- **WHEN** a release candidate is validated
- **THEN** the matrix covers primary and secondary displays, negative origins, Windows 100/150/200 percent scaling, Retina scaling, and display hot-plug recovery

#### Scenario: Capture workflows are tested
- **WHEN** a release candidate is validated
- **THEN** the matrix covers permission grant/deny/revoke, region capture, window capture, stale sources, Subtitle Overlay exclusion, session replacement, and protected-content failure behavior

#### Scenario: Package lifecycle is tested
- **WHEN** a release candidate is validated
- **THEN** installation, first launch, upgrade, relaunch after permission changes, and uninstall are tested on both platforms

### Requirement: Platform-accurate documentation
Build and release documentation MUST distinguish host-local bundle targets from operating-system targets.

#### Scenario: Developer reads build instructions on macOS
- **WHEN** the developer follows the macOS build instructions
- **THEN** the documentation describes only macOS artifacts and points to native CI for Windows artifacts

#### Scenario: Developer reads build instructions on Windows
- **WHEN** the developer follows the Windows build instructions
- **THEN** the documentation describes only Windows artifacts and does not claim to produce macOS bundles
