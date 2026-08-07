## 1. Contracts and domain behavior

- [x] 1.1 Add serializable layout, selection, translation configuration, task state, and structured error models in Rust and TypeScript
- [x] 1.2 Implement and test normalized geometry validation, monitor-scale crop conversion, reading-order sorting, stable block IDs, and basic block classification

## 2. OCR and translation adapters

- [ ] 2.1 Add the target-gated macOS Vision OCR adapter and explicit unsupported adapter for other platforms
- [ ] 2.2 Add the bounded OpenAI-compatible translation client with endpoint, credential, response-size, and exact-ID validation
- [x] 2.3 Remove the real-looking default API key and expose in-memory translation configuration to the capture workflow

## 3. Capture translation service

- [ ] 3.1 Implement the single-active-task service that pre-captures monitor images, owns transient configuration, and cleans up every terminal path
- [ ] 3.2 Add narrow Tauri commands for starting, reading selector payload, completing, and cancelling capture translation
- [ ] 3.3 Create temporary per-monitor selector windows before selection and restore the main window on completion, failure, or cancellation

## 4. Frontend workflow and presentation

- [ ] 4.1 Add a typed frontend service and composable with deterministic idle, selecting, processing, empty, success, error, and cancelled states
- [ ] 4.2 Add the full-window selector entry with frozen background, pointer selection, dimensions, Escape cancellation, and keyboard-safe cleanup
- [ ] 4.3 Add the layout-preserving result workspace with original, translated, and bilingual modes plus bounded responsive text blocks
- [ ] 4.4 Connect the custom-region action to the real workflow and remove the timer-based simulated capture success

## 5. Verification

- [ ] 5.1 Add focused tests for DTO validation, coordinate conversion, translation ID mapping, empty OCR, cancellation, and stale task handling
- [ ] 5.2 Run frontend build, Rust format/check/test, browser visual checks, and a native macOS capture/OCR smoke test; document any platform gaps
