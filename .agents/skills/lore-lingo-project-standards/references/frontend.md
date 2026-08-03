# Frontend Standards

## Stack and Boundaries

- Use Vue 3 Composition API, TypeScript strict mode, Vite, and Tauri 2 APIs.
- Use Naive UI as the shared interactive component library. Read `frontend-naive-ui.md` before adding or replacing UI components.
- Use Lucide Vue for Vue-rendered interface icons. Do not add a second general-purpose icon library.
- Prefer `<script setup lang="ts">` for Vue components.
- Keep platform and database implementation details behind typed frontend services.
- Use runtime capability data for Windows/macOS differences; do not parse the user agent.
- Keep macOS and Windows presentation behind one component contract and separate platform themes; do not fork feature behavior by platform.

## Components

- Give each component one clear UI responsibility.
- Define typed props and emits. Avoid `any`; use `unknown` plus validation at external boundaries.
- Keep components declarative. Move reusable state/lifecycle orchestration into composables.
- Clean up event listeners, timers, subscriptions, and Tauri unlisten functions in `onUnmounted`.
- Provide keyboard operation, accessible names, visible focus, and appropriate semantic elements.
- Render explicit loading, empty, denied, unavailable, error, and success states where relevant.

## State and IPC

- Keep state local unless multiple components or windows genuinely share it.
- Treat Tauri events as cross-window notifications, not a durable source of truth.
- Centralize direct `invoke`, window, and event APIs in typed service modules as contracts mature.
- Serialize stable DTOs only; never expose `HWND`, Core Graphics objects, raw paths, or Rust error strings.
- Prevent feedback loops when mirroring state across windows and test the chosen guard.

## Async Behavior

- Make cancellation and stale-response handling explicit for capture, OCR, translation, and search.
- Do not leave rejected promises silently swallowed unless failure is intentionally best-effort and documented.
- Prevent duplicate submissions and unbounded event/frame queues.
- Use deterministic status enums rather than unrelated booleans for multi-state workflows.

## Styling

- Keep design tokens and cross-window defaults in `src/styles/`; keep feature styles scoped to components.
- Treat Naive UI defaults as implementation scaffolding, not the LensLingo visual design.
- Read `frontend-macos-27.md` for macOS work and `frontend-windows-11.md` for Windows work.
- Provide an opaque/translucent fallback when platform transparency or backdrop blur differs.
- Use stable responsive dimensions for fixed-format controls and overlays.
- Avoid hard-coded desktop sizes and macOS-only symbols in cross-platform UI.
- Ensure text, controls, and focus states remain usable at Windows DPI scaling and macOS Retina scaling.

## Validation

- `pnpm build` is the current mandatory frontend baseline.
- Apply `frontend-visual-validation.md` to every material visual change.
- Add focused unit tests when introducing parsing, state machines, coordinate transforms, or error mapping.
- Add component/integration tests for permission, source-selection, multi-window, and recovery behavior.
- Verify browser preview only as a UI aid; native Tauri behavior requires native smoke testing.
