## ADDED Requirements

### Requirement: Explicit single-shot capture translation
The system SHALL start capture translation only after an explicit user action and SHALL own at most one active single-shot task.

#### Scenario: User starts a new task
- **WHEN** the user starts capture translation while no task is active
- **THEN** the system captures selector backgrounds before showing temporary selector windows and reports an active task identifier

#### Scenario: User replaces an active task
- **WHEN** the user starts a new capture translation while another task is active
- **THEN** the system cancels and cleans up the previous task before activating the replacement

#### Scenario: User cancels selection
- **WHEN** the user presses Escape or otherwise cancels the selector
- **THEN** every selector window is destroyed, transient images are released, and the main window is restored without reporting success

### Requirement: Monitor-local region selection
The system SHALL represent a selected region as a monitor identifier plus a monitor-local logical rectangle and SHALL crop the pre-captured image using the selected monitor scale.

#### Scenario: Region is selected on a Retina display
- **WHEN** the selector returns a valid logical rectangle on a scaled display
- **THEN** the system converts its start with bounded floor rounding, its end with bounded ceil rounding, and crops the matching physical pixels

#### Scenario: Region is invalid
- **WHEN** the selection has no area, exceeds the owning monitor, or exceeds configured image bounds
- **THEN** the system rejects it with `invalid_region` and cleans up the selector task

### Requirement: Structured OCR layout document
The system SHALL represent recognized content as a layout document containing image dimensions and ordered text blocks with stable IDs, normalized top-left-origin bounds, source text, block type, and confidence.

#### Scenario: Text is recognized
- **WHEN** OCR returns one or more valid text observations
- **THEN** the system normalizes their bounds, assigns stable IDs and reading order, and retains their spatial separation in the layout document

#### Scenario: No text is recognized
- **WHEN** OCR succeeds but returns no usable observations
- **THEN** the task terminates with an explicit empty result and does not call the translation provider

#### Scenario: OCR backend is unavailable
- **WHEN** capture translation runs on a platform without a production OCR adapter
- **THEN** the system returns `unsupported_capability` and does not synthesize OCR content

### Requirement: ID-preserving structured translation
The system MUST translate bounded groups of layout blocks while preserving every input block ID and MUST reject incomplete, duplicated, or unknown output IDs.

#### Scenario: Translation succeeds
- **WHEN** the provider returns exactly one valid translation for every submitted block ID
- **THEN** the system attaches each translated text to the matching layout block without changing geometry or reading order

#### Scenario: Translation configuration is missing
- **WHEN** a required endpoint, model, or remote API key is absent
- **THEN** the system returns `translation_not_configured` before sending recognized text to a network destination

#### Scenario: Provider response cannot be mapped safely
- **WHEN** the provider omits an ID, duplicates an ID, adds an unknown ID, or returns an oversized response
- **THEN** the system returns `translation_protocol_error` and does not display partially mapped translations as complete

### Requirement: Layout-preserving result presentation
The application SHALL present the captured image at a stable aspect ratio and SHALL position translated text from normalized block geometry without changing block identity or reading order.

#### Scenario: User views translated mode
- **WHEN** a completed layout document is shown in translated mode
- **THEN** each translated block is rendered over its original region with a readable surface, bounded text sizing, and no change to surrounding image geometry

#### Scenario: User changes result mode
- **WHEN** the user switches among original, translated, and bilingual modes
- **THEN** the same document and geometry are reused without rerunning OCR or translation

#### Scenario: Translation is longer than the source region
- **WHEN** translated text cannot fit at the preferred size
- **THEN** the block reduces text size within configured bounds, wraps text, and contains any remaining overflow without resizing or overlapping adjacent blocks

### Requirement: Private and recoverable task lifecycle
The system MUST keep screenshots, recognized text, translations, and API credentials out of persistent browser and application storage during this workflow and SHALL expose deterministic task states and safe errors.

#### Scenario: Task reaches a terminal state
- **WHEN** a task succeeds, fails, or is cancelled
- **THEN** native selectors and workers are cleaned up, the main window is restored, and the frontend reaches exactly one terminal state

#### Scenario: Diagnostics are emitted
- **WHEN** OCR or translation fails
- **THEN** user-facing IPC omits API credentials, authorization headers, captured text, native handles, local paths, and raw provider responses

#### Scenario: User retries after failure
- **WHEN** the user retries a recoverable failure
- **THEN** the system starts a new task using current in-memory configuration and does not reuse stale selector or result state
