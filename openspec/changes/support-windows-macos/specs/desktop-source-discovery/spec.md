## ADDED Requirements

### Requirement: Runtime desktop capabilities
The system SHALL expose the runtime operating system, architecture, permission model, and supported capture and presentation capabilities through one platform-neutral response.

#### Scenario: Windows capabilities are requested
- **WHEN** the frontend requests desktop capabilities on a supported Windows host
- **THEN** the response identifies Windows and reports each supported capability without requiring user-agent parsing

#### Scenario: macOS capabilities are requested
- **WHEN** the frontend requests desktop capabilities on a supported macOS host
- **THEN** the response identifies macOS and includes screen-capture permission and restart behavior

### Requirement: Screen discovery
The system SHALL enumerate every connected display with an opaque monitor ID, primary flag, logical size, physical size, scale factor, and virtual-desktop physical origin.

#### Scenario: Secondary display has a negative origin
- **WHEN** a display is positioned left of or above the primary display
- **THEN** discovery returns its negative physical origin without discarding or normalizing the display

#### Scenario: Displays use different scale factors
- **WHEN** connected displays use different DPI or Retina scale factors
- **THEN** each display reports its own logical and physical dimensions and scale factor

### Requirement: Window source discovery
The system SHALL enumerate capturable top-level application windows on Windows and macOS using a common `CaptureSource` model with opaque identity and best-effort application metadata.

#### Scenario: Capturable windows are available
- **WHEN** the user opens the window source picker with capture permission available
- **THEN** the catalog contains visible titled application windows with title, application name, optional icon, and capturable status

#### Scenario: Metadata cannot be read
- **WHEN** process or icon metadata is inaccessible for an otherwise capturable window
- **THEN** the window remains in the catalog with missing optional metadata instead of failing the entire discovery request

#### Scenario: LoreLingo windows are enumerated by the operating system
- **WHEN** native discovery returns LoreLingo's main, subtitle, or selector windows
- **THEN** those windows are excluded from the user-selectable source catalog

### Requirement: Catalog-scoped source identity
The system MUST scope source identifiers to a catalog revision and MUST reject a source that cannot be resolved against the active revision.

#### Scenario: A source is selected from the current catalog
- **WHEN** capture starts with a source ID and matching active catalog revision
- **THEN** the service resolves the source to its native representation without exposing that representation to Vue

#### Scenario: A window closes after discovery
- **WHEN** capture starts with a source from a stale catalog or a native window that no longer exists
- **THEN** the service returns `source_stale` or `source_not_found` and does not capture a different reused handle

### Requirement: Capture permission lifecycle
The system SHALL expose capture permission status and SHALL request permission only in response to an explicit user action.

#### Scenario: macOS permission has not been decided
- **WHEN** the user explicitly starts discovery or capture and permission is not determined
- **THEN** the system requests screen-recording permission and reports the resulting status and required next action

#### Scenario: macOS permission is denied
- **WHEN** permission is denied or revoked
- **THEN** window capture controls are disabled and the response provides an action to open the relevant system settings without pretending that no windows exist

#### Scenario: Windows does not require a global prompt
- **WHEN** permission status is requested on Windows
- **THEN** the system reports the Windows permission model while preserving per-source accessibility failures as structured source errors

### Requirement: Discovery refresh after topology changes
The system SHALL replace the active catalog when displays or windows are refreshed and SHALL publish a new catalog revision.

#### Scenario: A display is connected or removed
- **WHEN** the operating-system display topology changes and discovery is refreshed
- **THEN** the new catalog reflects the current displays and invalidates source selections tied to removed displays

