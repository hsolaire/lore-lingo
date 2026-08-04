## ADDED Requirements

### Requirement: Platform-correct main window controls
The main window SHALL present and operate controls according to the runtime desktop platform while preserving a shared Vue component contract.

#### Scenario: Main window runs on macOS
- **WHEN** desktop capabilities identify macOS
- **THEN** close and minimize controls use the macOS placement and interaction convention

#### Scenario: Main window runs on Windows
- **WHEN** desktop capabilities identify Windows
- **THEN** minimize and close controls use the Windows placement and interaction convention and macOS-only decoration is not shown

#### Scenario: User drags the custom title bar
- **WHEN** the user drags a non-interactive title-bar area
- **THEN** the native window moves without making title-bar buttons initiate a drag

### Requirement: Content-sized subtitle overlay
The subtitle overlay SHALL use a native window bounded to its visible content rather than an invisible desktop-sized input surface.

#### Scenario: Subtitle overlay is shown
- **WHEN** the user opens the Subtitle Overlay
- **THEN** a transparent always-on-top window is positioned around the subtitle content and unrelated desktop areas remain interactive

#### Scenario: Subtitle content changes height
- **WHEN** wrapping or controls change the measured subtitle content size
- **THEN** the native Subtitle Overlay is resized and clamped without moving off the active monitor

#### Scenario: User closes the Subtitle Overlay
- **WHEN** the Subtitle Overlay is closed
- **THEN** the Subtitle Overlay is hidden or destroyed and does not retain an invisible input-blocking window

### Requirement: Monitor-aware Subtitle Overlay persistence
The system SHALL persist Subtitle Overlay placement relative to a monitor and SHALL recover safely when the stored topology no longer exists.

#### Scenario: Stored monitor remains connected
- **WHEN** the Subtitle Overlay is restored and its stored monitor is available
- **THEN** its position and width are restored and clamped to that monitor's logical work area

#### Scenario: Stored monitor was removed
- **WHEN** the Subtitle Overlay is restored after its stored monitor has been disconnected
- **THEN** it moves to a visible clamped position on the primary monitor

#### Scenario: Scale factor changes
- **WHEN** the Subtitle Overlay moves between monitors with different scale factors
- **THEN** its logical size remains usable and its native physical bounds are recalculated

### Requirement: Temporary per-monitor region selectors
Region selection SHALL use temporary borderless selector windows associated with concrete monitor IDs.

#### Scenario: Region selection begins with multiple monitors
- **WHEN** the user starts region selection with multiple displays connected
- **THEN** a selector surface covers each eligible display and reports monitor-local logical coordinates

#### Scenario: Region selection completes or is cancelled
- **WHEN** the user confirms or cancels selection
- **THEN** every selector window is destroyed before capture starts

### Requirement: Presentation fallback
The application SHALL remain readable and operable when transparent-window or backdrop effects differ between WebView2 and WKWebView.

#### Scenario: Backdrop effect is unavailable
- **WHEN** the runtime cannot render the intended transparent blur effect correctly
- **THEN** the window uses an opaque or translucent fallback with sufficient contrast and unchanged controls
