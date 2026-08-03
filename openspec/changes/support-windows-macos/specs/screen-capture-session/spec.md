## ADDED Requirements

### Requirement: Monitor-local region contract
The system SHALL represent a region as a monitor ID and a rectangle in monitor-local logical pixels, and Rust SHALL perform the logical-to-physical conversion.

#### Scenario: Region is captured on a scaled display
- **WHEN** a valid logical rectangle is submitted for a display with a non-unit scale factor
- **THEN** the captured physical bounds use that display's scale transform with bounded rounding and match the selected content

#### Scenario: Region is captured on a negative-origin display
- **WHEN** a valid local rectangle is submitted for a display with a negative virtual-desktop origin
- **THEN** capture succeeds without treating the local coordinates as invalid or applying the origin twice

#### Scenario: Region exceeds display bounds
- **WHEN** a requested region is empty or extends outside its selected monitor
- **THEN** the service rejects it with `invalid_region` rather than silently capturing a different rectangle

### Requirement: Single-monitor region selection
The first release MUST keep each selected region wholly within one display.

#### Scenario: Drag approaches a monitor boundary
- **WHEN** the user drags a region toward another display
- **THEN** the selector clamps the region to the display where the drag began

#### Scenario: Cross-display request bypasses the selector
- **WHEN** a client submits a region that requires multiple displays
- **THEN** the service returns `cross_display_region_unsupported`

### Requirement: Window capture session
The system SHALL start a capture session from a current capturable window source on both supported operating systems.

#### Scenario: Current window source starts successfully
- **WHEN** a current source ID, matching catalog revision, and supported frame rate are submitted
- **THEN** the service returns a session ID and begins producing timestamped frames for that window

#### Scenario: Window becomes unavailable
- **WHEN** the selected window closes, is no longer capturable, or changes into a protected state
- **THEN** the session enters a failed or stopped state with a structured recoverable error

### Requirement: Capture session lifecycle
The system SHALL expose explicit start, status, and stop operations and SHALL support at most one active capture session in the initial release.

#### Scenario: User stops an active session
- **WHEN** stop is requested for the active session
- **THEN** its worker is cancelled, native resources are released, and the terminal status is observable

#### Scenario: User starts a replacement session
- **WHEN** a new session is started while another session is active
- **THEN** the previous session is stopped before the replacement becomes active

#### Scenario: Application exits with a session active
- **WHEN** LensLingo exits or its Tauri state is dropped
- **THEN** capture workers are cancelled and native resources are released

### Requirement: Bounded latest-frame delivery
The capture coordinator MUST bound retained frame data and MUST prefer the latest frame over an unbounded backlog.

#### Scenario: Consumer is slower than capture
- **WHEN** frame production is faster than a future OCR consumer
- **THEN** intermediate frames are replaced or dropped and memory use remains bounded

#### Scenario: Frontend requests a preview
- **WHEN** a preview is requested for an active session
- **THEN** the service encodes the latest available frame as PNG without enabling continuous raw-frame IPC

### Requirement: LensLingo content exclusion
The capture backend SHALL exclude LensLingo's own persistent windows from captured frames when the operating system supports exclusion.

#### Scenario: Lyric overlay overlaps the source
- **WHEN** the lyric overlay is visible over a captured screen or window
- **THEN** the backend excludes it from the captured content when supported and reports self-exclusion capability accurately

#### Scenario: Backend cannot guarantee exclusion
- **WHEN** the active fallback backend cannot exclude LensLingo windows
- **THEN** the capability response reports that limitation and the application avoids claiming guaranteed self-exclusion

### Requirement: Structured capture failures
The system SHALL return stable failure codes and safe recovery metadata instead of raw native errors.

#### Scenario: Capture permission is denied
- **WHEN** a session cannot start because permission is denied
- **THEN** it returns `permission_denied` with a settings action and no native handle details

#### Scenario: Content is protected or blank
- **WHEN** the operating system refuses protected content or sustained blank output is detected
- **THEN** the session reports `protected_content` or `source_unavailable` and stops retrying indefinitely

