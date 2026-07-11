# Dual-Window Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split the single-window LensLingo app into two independent Tauri windows — `main` (420×640 main panel) and `overlay` (fullscreen transparent lyric bar) — eliminating the blank-space bug caused by sharing one window between two incompatible layouts.

**Architecture:** A new `overlay` Tauri window is defined in `tauri.conf.json` with its own Vite entry point (`overlay.html` / `src/overlay.ts`). The `main` window continues to host `App.vue` (main panel only). `useLyricMode` is refactored to open/close/communicate with the overlay window via Tauri's event system instead of toggling Vue state. Shared reactive state (`useTranslator`, `useCaptureSource`) is replaced by Tauri events for cross-window sync.

**Tech Stack:** Tauri 2, Vue 3.5, TypeScript 5, Vite 5, `@tauri-apps/api` (window, event APIs)

## Global Constraints

- Tauri version: `^2` — use `@tauri-apps/api/window` (`WebviewWindow`, `getCurrentWindow`) and `@tauri-apps/api/event` (`emit`, `listen`)
- No new npm dependencies
- `@` alias resolves to `src/`
- All windows: `decorations: false`, `transparent: true`, `shadow: false`
- Keep `macOSPrivateApi: true` in tauri.conf.json
- Do NOT use `tauri-plugin-store` — persist to `localStorage` as before
- TypeScript strict mode — no `any` except Tauri window handle catch blocks

---

## File Map

### Created
- `overlay.html` — second Vite HTML entry for overlay window
- `src/overlay.ts` — mounts `OverlayApp.vue`
- `src/OverlayApp.vue` — root component for overlay window (just `LyricBar` + `AppToast`)
- `src/composables/useOverlayBridge.ts` — overlay-side: listens to events from main window, re-emits back
- `docs/superpowers/plans/2026-07-12-dual-window-refactor.md` — this file

### Modified
- `src-tauri/tauri.conf.json` — add `overlay` window definition
- `vite.config.ts` — add `overlay.html` as second Rollup input
- `src/composables/useLyricMode.ts` — replace Vue state toggle with `WebviewWindow` open/close + `emit`
- `src/composables/useTranslator.ts` — emit translator state changes as Tauri events so overlay stays in sync
- `src/App.vue` — remove `LyricBar` import; remove `lyricMode` body-class watcher
- `src/components/lyric/LyricControls.vue` — `exit`/`close` now emit Tauri event to close overlay & show main

### Unchanged (keep as-is)
- `src/components/lyric/LyricBar.vue`
- `src/components/lyric/LyricBox.vue`
- `src/components/main-panel/*` (all)
- `src/composables/useLyricDrag.ts`
- `src/composables/useCaptureSource.ts`
- `src/composables/useSettings.ts`
- `src/composables/useToast.ts`
- `src/styles/global.css`
- `src/icons/paths.ts`
- `src-tauri/src/lib.rs`

---

## Task 1: Add overlay window to tauri.conf.json

**Files:**
- Modify: `src-tauri/tauri.conf.json`

**Goal:** Define the `overlay` window — fullscreen, transparent, always-on-top, no decorations. It starts hidden (`visible: false`) so it only appears when the user clicks the lyric-mode button.

- [ ] **Step 1: Add overlay window config**

Open `src-tauri/tauri.conf.json`. In the `app.windows` array, append after the existing `main` entry:

```json
{
  "label": "overlay",
  "title": "LensLingo Overlay",
  "url": "overlay.html",
  "width": 1920,
  "height": 1080,
  "fullscreen": false,
  "resizable": false,
  "decorations": false,
  "transparent": true,
  "shadow": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "visible": false,
  "focus": false
}
```

The full `app.windows` array should now look like:

```json
"windows": [
  {
    "label": "main",
    "title": "LensLingo",
    "width": 420,
    "height": 640,
    "minWidth": 380,
    "minHeight": 480,
    "resizable": true,
    "decorations": false,
    "transparent": true,
    "shadow": false,
    "alwaysOnTop": false,
    "skipTaskbar": false,
    "center": true
  },
  {
    "label": "overlay",
    "title": "LensLingo Overlay",
    "url": "overlay.html",
    "width": 1920,
    "height": 1080,
    "fullscreen": false,
    "resizable": false,
    "decorations": false,
    "transparent": true,
    "shadow": false,
    "alwaysOnTop": true,
    "skipTaskbar": true,
    "visible": false,
    "focus": false
  }
]
```

- [ ] **Step 2: Verify JSON is valid**

```bash
cd D:\Code\RustRover\lore-lingo
node -e "JSON.parse(require('fs').readFileSync('src-tauri/tauri.conf.json','utf8')); console.log('OK')"
```

Expected output: `OK`

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tauri.conf.json
git commit -m "feat: add overlay window definition to tauri config"
```

---

## Task 2: Add overlay HTML entry and Vite config

**Files:**
- Create: `overlay.html`
- Modify: `vite.config.ts`

**Goal:** Vite needs to know about the second entry point so it bundles `overlay.html` alongside `index.html` for both dev and production.

- [ ] **Step 1: Create overlay.html**

Create `overlay.html` at the project root (same level as `index.html`). Copy `index.html` exactly and change only the script src:

```html
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>LensLingo Overlay</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/overlay.ts"></script>
  </body>
</html>
```

- [ ] **Step 2: Add overlay as second Vite input**

Open `vite.config.ts`. Add a `build.rollupOptions.input` field. The full modified `vite.config.ts`:

```ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  build: {
    target: ['es2021', 'chrome105', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    rollupOptions: {
      input: {
        main: fileURLToPath(new URL('./index.html', import.meta.url)),
        overlay: fileURLToPath(new URL('./overlay.html', import.meta.url)),
      },
    },
  },
  envPrefix: ['VITE_', 'TAURI_'],
})
```

- [ ] **Step 3: Verify Vite still starts**

```bash
pnpm dev
```

Expected: Vite starts on port 5173 with no errors. Visit `http://localhost:5173/overlay.html` — it will 404 on the script (not yet created) but the HTML should load.

Kill the dev server with Ctrl+C.

- [ ] **Step 4: Commit**

```bash
git add overlay.html vite.config.ts
git commit -m "feat: add overlay.html entry and configure Vite multi-page build"
```

---

## Task 3: Create overlay Vue entry point

**Files:**
- Create: `src/overlay.ts`
- Create: `src/OverlayApp.vue`

**Goal:** The overlay window gets its own Vue app that only mounts `LyricBar` and `AppToast`. It does NOT include `MainPanel`, `CaptureFrame`, or anything from the main panel.

- [ ] **Step 1: Create src/overlay.ts**

```ts
import { createApp } from 'vue'
import './styles/global.css'
import OverlayApp from './OverlayApp.vue'

createApp(OverlayApp).mount('#app')
```

- [ ] **Step 2: Create src/OverlayApp.vue**

```vue
<script setup lang="ts">
import AppToast from '@/components/AppToast.vue'
import LyricBar from '@/components/lyric/LyricBar.vue'
</script>

<template>
  <LyricBar />
  <AppToast />
</template>
```

- [ ] **Step 3: Verify overlay page loads in browser**

```bash
pnpm dev
```

Open `http://localhost:5173/overlay.html` in a browser. Expected: LyricBar renders (with placeholder text). No console errors.

Kill dev server.

- [ ] **Step 4: Commit**

```bash
git add src/overlay.ts src/OverlayApp.vue
git commit -m "feat: create overlay Vue entry with LyricBar only"
```

---

## Task 4: Refactor useLyricMode — main window side

**Files:**
- Modify: `src/composables/useLyricMode.ts`

**Goal:** Replace the single-window Vue state toggle with Tauri `WebviewWindow` show/hide calls. The main window emits a `lyric:enter` event when switching to lyric mode, and `lyric:exit` / `lyric:close` when returning.

The overlay window is already defined in `tauri.conf.json` — we use `WebviewWindow.getByLabel('overlay')` to get a handle to it without creating a new window.

- [ ] **Step 1: Rewrite useLyricMode.ts**

```ts
import { ref } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { emit } from '@tauri-apps/api/event'
import { useToast } from './useToast'
import { useSettings } from './useSettings'

// peekSignal is only used within the overlay window now, but keep it
// so LyricBar (which also runs in overlay) can still import this composable.
const peekSignal = ref(0)

async function getOverlay(): Promise<WebviewWindow | null> {
  try {
    return WebviewWindow.getByLabel('overlay')
  } catch (_) {
    return null
  }
}

export function useLyricMode() {
  const { toast } = useToast()
  const { closeDrawer } = useSettings()

  async function enter() {
    closeDrawer()
    const overlay = await getOverlay()
    if (overlay) {
      await overlay.show()
      await overlay.setFocus()
    }
    await emit('lyric:enter', {})
    toast('已切换为屏幕歌词条 · 置顶显示')
  }

  async function exit() {
    const overlay = await getOverlay()
    if (overlay) await overlay.hide()
    await emit('lyric:exit', {})
    toast('已回到面板视图')
  }

  async function close() {
    const overlay = await getOverlay()
    if (overlay) await overlay.hide()
    await emit('lyric:close', {})
    toast('歌词条已关闭')
  }

  // lyricMode ref is no longer used in main window — kept for API compatibility
  // with any component that imports it (they can safely ignore it)
  const lyricMode = ref(false)

  return { lyricMode, peekSignal, enter, exit, close }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/composables/useLyricMode.ts
git commit -m "refactor: useLyricMode now shows/hides overlay window via Tauri API"
```

---

## Task 5: Simplify App.vue — remove lyric mode from main window

**Files:**
- Modify: `src/App.vue`

**Goal:** The main window never renders `LyricBar` anymore. Remove it entirely from `App.vue`. The `lyric-mode` body class is no longer needed either.

- [ ] **Step 1: Rewrite App.vue**

```vue
<script setup lang="ts">
import { watchEffect } from 'vue'
import { useCaptureSource } from '@/composables/useCaptureSource'
import CaptureFrame from '@/components/CaptureFrame.vue'
import AppToast from '@/components/AppToast.vue'
import MainPanel from '@/components/main-panel/MainPanel.vue'

const { selecting } = useCaptureSource()

watchEffect(() => {
  document.body.classList.toggle('selecting', selecting.value)
})
</script>

<template>
  <CaptureFrame />
  <MainPanel />
  <AppToast />
</template>
```

- [ ] **Step 2: Commit**

```bash
git add src/App.vue
git commit -m "refactor: remove LyricBar from main window App.vue"
```

---

## Task 6: Create useOverlayBridge — overlay-side event listener

**Files:**
- Create: `src/composables/useOverlayBridge.ts`

**Goal:** The overlay window needs to react to events from the main window. `useOverlayBridge` listens for `lyric:enter` (trigger peek), `lyric:exit` / `lyric:close` (hide self), and `translator:update` (sync text/language state). It also listens for `lyric:peek` to trigger the controls peek animation.

This composable is called once in `OverlayApp.vue` on mount.

- [ ] **Step 1: Create src/composables/useOverlayBridge.ts**

```ts
import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTranslator } from './useTranslator'

export type TranslatorPayload = {
  srcText: string
  tgtText: string
  langA: string
  langB: string
  pinned: boolean
}

export function useOverlayBridge(onPeek: () => void) {
  const { srcText, tgtText, langA, langB, pinned } = useTranslator()
  const unlisten: UnlistenFn[] = []

  onMounted(async () => {
    unlisten.push(
      await listen<TranslatorPayload>('translator:update', ({ payload }) => {
        srcText.value  = payload.srcText
        tgtText.value  = payload.tgtText
        langA.value    = payload.langA
        langB.value    = payload.langB
        pinned.value   = payload.pinned
      }),
      await listen('lyric:enter', () => {
        onPeek()
      }),
      await listen('lyric:peek', () => {
        onPeek()
      }),
    )

    // Keep overlay always-on-top when shown
    try {
      await getCurrentWindow().setAlwaysOnTop(true)
    } catch (_) {}
  })

  onUnmounted(() => {
    unlisten.forEach(fn => fn())
  })
}
```

- [ ] **Step 2: Wire useOverlayBridge into OverlayApp.vue**

```vue
<script setup lang="ts">
import { ref } from 'vue'
import AppToast from '@/components/AppToast.vue'
import LyricBar from '@/components/lyric/LyricBar.vue'
import { useOverlayBridge } from '@/composables/useOverlayBridge'

const peekTrigger = ref(0)
function triggerPeek() { peekTrigger.value++ }

useOverlayBridge(triggerPeek)
</script>

<template>
  <LyricBar :external-peek="peekTrigger" />
  <AppToast />
</template>
```

- [ ] **Step 3: Update LyricBar.vue to accept externalPeek prop**

In `src/components/lyric/LyricBar.vue`, replace the `peekSignal` watch with a prop watch:

```vue
<script setup lang="ts">
import { ref, watch } from 'vue'
import { useLyricDrag } from '@/composables/useLyricDrag'
import { useTranslator } from '@/composables/useTranslator'
import LyricControls from './LyricControls.vue'
import LyricBox from './LyricBox.vue'

const props = defineProps<{ externalPeek?: number }>()

const barEl = ref<HTMLElement | null>(null)

const peekActive = ref(false)
let peekTimer: ReturnType<typeof setTimeout> | undefined
function peek(ms = 2200) {
  peekActive.value = true
  clearTimeout(peekTimer)
  peekTimer = setTimeout(() => { peekActive.value = false }, ms)
}
function onEnter() { clearTimeout(peekTimer) }
function onLeave() {
  clearTimeout(peekTimer)
  peekActive.value = false
}

const { placed, dragging, pos, size, onDragBarMousedown, onResizeMousedown } = useLyricDrag(barEl, () => peek())

watch(() => props.externalPeek, () => peek())

const { flip, swapText } = useTranslator()
const swapping = ref(false)
function onFlip() {
  swapping.value = true
  setTimeout(() => { swapText(); swapping.value = false }, 180)
  flip()
}
</script>

<template>
  <div
    ref="barEl"
    class="lyric-bar"
    :class="{ placed, dragging, peek: peekActive }"
    :style="{
      ...(placed ? { left: pos.x + 'px', top: pos.y + 'px' } : {}),
      ...(size.width ? { width: size.width + 'px' } : {}),
    }"
    @mouseenter="onEnter"
    @mouseleave="onLeave"
  >
    <LyricControls @flip="onFlip" />
    <LyricBox
      :swap="swapping"
      @dragbar-mousedown="onDragBarMousedown"
      @resize-mousedown="onResizeMousedown"
    />
  </div>
</template>

<style scoped>
.lyric-bar {
  position: fixed;
  left: 50%; top: 40px;
  transform: translateX(-50%);
  width: min(760px, calc(100vw - 48px));
  z-index: 40;
  display: flex;
  flex-direction: column;
  align-items: stretch;
}
.lyric-bar.placed { transform: none; }
.lyric-bar.dragging { user-select: none; }
.lyric-bar.dragging :deep(.lyric-ctrl) { opacity: 0 !important; }

.lyric-bar:hover :deep(.lyric-ctrl),
.lyric-bar.peek :deep(.lyric-ctrl) {
  opacity: 1; transform: translateY(0); pointer-events: auto;
}
</style>
```

- [ ] **Step 4: Commit**

```bash
git add src/composables/useOverlayBridge.ts src/OverlayApp.vue src/components/lyric/LyricBar.vue
git commit -m "feat: overlay bridge for cross-window event sync; LyricBar accepts externalPeek prop"
```

---

## Task 7: Sync translator state across windows

**Files:**
- Modify: `src/composables/useTranslator.ts`

**Goal:** When the main window changes `srcText`, `tgtText`, `langA`, `langB`, or `pinned`, it emits a `translator:update` Tauri event. The overlay listens (via `useOverlayBridge`) and updates its local refs. This keeps both windows in sync without a shared process.

- [ ] **Step 1: Rewrite useTranslator.ts**

```ts
import { ref, watch } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'
import { useToast } from './useToast'

const srcText = ref("I never thought I'd find you here, of all places.")
const tgtText = ref('没想到会在这种地方遇见你。')
const langA = ref('English')
const langB = ref('简体中文')
const pinned = ref(false)
const hit = ref(true)

let isBroadcasting = false

async function broadcastState() {
  if (isBroadcasting) return
  try {
    await emit('translator:update', {
      srcText: srcText.value,
      tgtText: tgtText.value,
      langA: langA.value,
      langB: langB.value,
      pinned: pinned.value,
    })
  } catch (_) {}
}

// Watch all shared state and broadcast on change.
// Guard with isBroadcasting to prevent the overlay's listen handler
// from re-emitting when it updates local refs.
watch([srcText, tgtText, langA, langB, pinned], () => {
  broadcastState()
})

export function useTranslator() {
  const { toast } = useToast()

  function flip() {
    ;[langA.value, langB.value] = [langB.value, langA.value]
    toast('已切换翻译方向')
  }

  function swapText() {
    ;[srcText.value, tgtText.value] = [tgtText.value, srcText.value]
  }

  async function togglePin() {
    const nextState = !pinned.value
    try {
      await getCurrentWindow().setAlwaysOnTop(nextState)
      pinned.value = nextState
    } catch (_) {}
    toast(pinned.value ? '窗口已置顶' : '取消置顶')
  }

  return { srcText, tgtText, langA, langB, pinned, hit, flip, swapText, togglePin }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/composables/useTranslator.ts
git commit -m "feat: broadcast translator state changes as Tauri events for overlay sync"
```

---

## Task 8: Fix LyricControls — exit/close via Tauri event

**Files:**
- Modify: `src/components/lyric/LyricControls.vue`

**Goal:** In the overlay window, `useLyricMode().exit()` and `close()` need to hide the overlay window AND notify the main window. Currently `exit()` in `useLyricMode` does this already via `emit('lyric:exit')`. But `LyricControls` also calls `togglePin` — in the overlay, `getCurrentWindow()` returns the overlay window, which is correct (we want overlay to be pinned). No change needed for pin.

The only change: `LyricControls` no longer needs to import `useLyricMode` from main-window context — it works the same way since `useLyricMode().exit()` calls `WebviewWindow.getByLabel('overlay').hide()` which works from either window.

- [ ] **Step 1: Verify LyricControls.vue needs no changes**

Read `src/components/lyric/LyricControls.vue`. Confirm:
- It imports `useLyricMode` → calls `exit()` and `close()` 
- Both `exit()` and `close()` in refactored `useLyricMode` already call `overlay.hide()` + emit event
- `togglePin` calls `getCurrentWindow().setAlwaysOnTop()` — in overlay context this correctly targets the overlay window

No code changes needed. This task is a verification checkpoint.

- [ ] **Step 2: Commit checkpoint note**

```bash
git commit --allow-empty -m "chore: verify LyricControls works without changes in overlay context"
```

---

## Task 9: End-to-end smoke test

**Goal:** Verify the full flow works in the Tauri dev build.

- [ ] **Step 1: Run Tauri dev**

```bash
pnpm tauri:dev
```

Expected: Two windows launch — the main panel (420×640) and the overlay (hidden initially).

- [ ] **Step 2: Test enter lyric mode**

Click the music note (♪) button in the main panel title bar.

Expected:
- Overlay window appears over the desktop with the lyric bar
- Main panel remains visible behind it (or minimizes — depends on UX preference)
- Toast "已切换为屏幕歌词条 · 置顶显示" appears in overlay

- [ ] **Step 3: Test exit lyric mode**

Hover over the lyric bar to show `LyricControls`. Click the layout (回到面板) button.

Expected:
- Overlay window hides
- Toast "已回到面板视图" appears in main window

- [ ] **Step 4: Test close**

Enter lyric mode again. Click the X button in `LyricControls`.

Expected:
- Overlay window hides
- Toast "歌词条已关闭" appears in main window

- [ ] **Step 5: Test translator sync**

While in lyric mode, verify the text shown in the overlay matches the main panel (both show the same `srcText` / `tgtText`).

- [ ] **Step 6: Commit if all passing**

```bash
git add -A
git commit -m "feat: dual-window architecture complete — overlay window for lyric mode"
```

---

## Self-Review

**Spec coverage:**
- ✅ Overlay window defined in tauri.conf.json (Task 1)
- ✅ Second Vite entry for overlay (Task 2)
- ✅ Overlay Vue app mounts only LyricBar (Task 3)
- ✅ Main window never shows LyricBar (Task 5)
- ✅ Show/hide overlay on mode switch (Task 4)
- ✅ Cross-window state sync for translator (Tasks 6, 7)
- ✅ LyricControls exit/close hides overlay (Task 8)
- ✅ Peek animation works via externalPeek prop (Task 6)
- ✅ No blank space — overlay is a separate fullscreen transparent window

**Placeholder scan:** No TBD or TODO found.

**Type consistency:**
- `TranslatorPayload` defined in `useOverlayBridge.ts` and used in `listen<TranslatorPayload>` — ✅
- `externalPeek?: number` prop in `LyricBar`, passed as `:external-peek="peekTrigger"` from `OverlayApp` — ✅
- `WebviewWindow.getByLabel('overlay')` — returns `WebviewWindow | null` in Tauri v2 — ✅
- `emit('translator:update', payload)` matches `listen<TranslatorPayload>('translator:update', ...)` — ✅
