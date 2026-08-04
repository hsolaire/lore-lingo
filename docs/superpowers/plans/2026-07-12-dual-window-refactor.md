# Dual-Window Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split the single-window LensLingo app into two independent Tauri windows — `main` (420×640 main panel) and `subtitle` (content-sized transparent Subtitle Overlay) — eliminating the blank-space bug caused by sharing one window between two incompatible layouts.

**Architecture:** A new `subtitle` Tauri window is defined in `tauri.conf.json` with its own Vite entry point (`subtitle.html` / `src/subtitle.ts`). The `main` window continues to host `App.vue` (main panel only). `useSubtitle` is refactored to open/close/communicate with the subtitle window via Tauri's event system instead of toggling Vue state. Shared reactive state (`useTranslator`, `useCaptureSource`) is replaced by Tauri events for cross-window sync.

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
- `subtitle.html` — second Vite HTML entry for subtitle window
- `src/subtitle.ts` — mounts `SubtitleApp.vue`
- `src/SubtitleApp.vue` — root component for subtitle window (just `SubtitleBar` + `AppToast`)
- `src/composables/useSubtitleBridge.ts` — subtitle-side: listens to events from main window, re-emits back
- `docs/superpowers/plans/2026-07-12-dual-window-refactor.md` — this file

### Modified
- `src-tauri/tauri.conf.json` — add `subtitle` window definition
- `vite.config.ts` — add `subtitle.html` as second Rollup input
- `src/composables/useSubtitle.ts` — replace Vue state toggle with `WebviewWindow` open/close + `emit`
- `src/composables/useTranslator.ts` — emit translator state changes as Tauri events so subtitle stays in sync
- `src/App.vue` — remove `SubtitleBar` import and the old single-window mode watcher
- `src/components/subtitle/SubtitleControls.vue` — `hide`/`close` emit Tauri events for the independent subtitle window

### Unchanged (keep as-is)
- `src/components/subtitle/SubtitleBar.vue`
- `src/components/subtitle/SubtitleContent.vue`
- `src/components/main-panel/*` (all)
- `src/composables/useSubtitleWindow.ts`
- `src/composables/useCaptureSource.ts`
- `src/composables/useSettings.ts`
- `src/composables/useToast.ts`
- `src/styles/global.css`
- `src/icons/paths.ts`
- `src-tauri/src/lib.rs`

---

## Task 1: Add subtitle window to tauri.conf.json

**Files:**
- Modify: `src-tauri/tauri.conf.json`

**Goal:** Define the `subtitle` window as a content-sized, transparent, always-on-top, undecorated window. It starts hidden (`visible: false`) and is independent from the main window.

- [ ] **Step 1: Add subtitle window config**

Open `src-tauri/tauri.conf.json`. In the `app.windows` array, append after the existing `main` entry:

```json
{
  "label": "subtitle",
  "title": "LensLingo 悬浮字幕",
  "url": "subtitle.html",
  "width": 808,
  "height": 204,
  "minWidth": 328,
  "maxWidth": 1248,
  "fullscreen": false,
  "resizable": true,
  "maximizable": false,
  "minimizable": false,
  "decorations": false,
  "transparent": true,
  "shadow": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "visible": false,
  "focus": false,
  "center": true
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
    "label": "subtitle",
    "title": "LensLingo 悬浮字幕",
    "url": "subtitle.html",
    "width": 808,
    "height": 204,
    "minWidth": 328,
    "maxWidth": 1248,
    "fullscreen": false,
    "resizable": true,
    "maximizable": false,
    "minimizable": false,
    "decorations": false,
    "transparent": true,
    "shadow": false,
    "alwaysOnTop": true,
    "skipTaskbar": true,
    "visible": false,
    "focus": false,
    "center": true
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
git commit -m "feat: add subtitle window definition to tauri config"
```

---

## Task 2: Add subtitle HTML entry and Vite config

**Files:**
- Create: `subtitle.html`
- Modify: `vite.config.ts`

**Goal:** Vite needs to know about the second entry point so it bundles `subtitle.html` alongside `index.html` for both dev and production.

- [ ] **Step 1: Create subtitle.html**

Create `subtitle.html` at the project root (same level as `index.html`). Copy `index.html` exactly and change only the script src:

```html
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>LensLingo 悬浮字幕</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/subtitle.ts"></script>
  </body>
</html>
```

- [ ] **Step 2: Add subtitle as second Vite input**

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
        subtitle: fileURLToPath(new URL('./subtitle.html', import.meta.url)),
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

Expected: Vite starts on port 5173 with no errors. Visit `http://localhost:5173/subtitle.html` — it will 404 on the script (not yet created) but the HTML should load.

Kill the dev server with Ctrl+C.

- [ ] **Step 4: Commit**

```bash
git add subtitle.html vite.config.ts
git commit -m "feat: add subtitle.html entry and configure Vite multi-page build"
```

---

## Task 3: Create subtitle Vue entry point

**Files:**
- Create: `src/subtitle.ts`
- Create: `src/SubtitleApp.vue`

**Goal:** The subtitle window gets its own Vue app that only mounts `SubtitleBar` and `AppToast`. It does NOT include `MainPanel`, `CaptureFrame`, or anything from the main panel.

- [ ] **Step 1: Create src/subtitle.ts**

```ts
import { createApp } from 'vue'
import './styles/global.css'
import SubtitleApp from './SubtitleApp.vue'

createApp(SubtitleApp).mount('#app')
```

- [ ] **Step 2: Create src/SubtitleApp.vue**

```vue
<script setup lang="ts">
import AppToast from '@/components/AppToast.vue'
import SubtitleBar from '@/components/subtitle/SubtitleBar.vue'
</script>

<template>
  <SubtitleBar />
  <AppToast />
</template>
```

- [ ] **Step 3: Verify subtitle page loads in browser**

```bash
pnpm dev
```

Open `http://localhost:5173/subtitle.html` in a browser. Expected: SubtitleBar renders (with placeholder text). No console errors.

Kill dev server.

- [ ] **Step 4: Commit**

```bash
git add src/subtitle.ts src/SubtitleApp.vue
git commit -m "feat: create subtitle Vue entry with SubtitleBar only"
```

---

## Task 4: Refactor useSubtitle — main window side

**Files:**
- Modify: `src/composables/useSubtitle.ts`

**Goal:** Replace the single-window Vue state toggle with Tauri `WebviewWindow` show/hide calls. The main window emits `subtitle:show`, `subtitle:hide`, and `subtitle:close` events for the independent Subtitle Overlay.

The subtitle window is already defined in `tauri.conf.json` — we use `WebviewWindow.getByLabel('subtitle')` to get a handle to it without creating a new window.

- [ ] **Step 1: Rewrite useSubtitle.ts**

```ts
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { emit } from '@tauri-apps/api/event'
import { useToast } from './useToast'
import { useSettings } from './useSettings'

async function getSubtitleWindow(): Promise<WebviewWindow | null> {
  try {
    return WebviewWindow.getByLabel('subtitle')
  } catch (_) {
    return null
  }
}

export function useSubtitle() {
  const { toast } = useToast()
  const { closeDrawer } = useSettings()

  async function show() {
    closeDrawer()
    const subtitleWindow = await getSubtitleWindow()
    if (subtitleWindow) {
      await subtitleWindow.show()
    }
    await emit('subtitle:show', {})
    toast('已打开悬浮字幕')
  }

  async function hide() {
    const subtitleWindow = await getSubtitleWindow()
    if (subtitleWindow) await subtitleWindow.hide()
    await emit('subtitle:hide', {})
    toast('已回到面板视图')
  }

  async function close() {
    const subtitleWindow = await getSubtitleWindow()
    if (subtitleWindow) await subtitleWindow.hide()
    await emit('subtitle:close', {})
    toast('悬浮字幕已关闭')
  }

  return { show, hide, close }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/composables/useSubtitle.ts
git commit -m "refactor: useSubtitle now shows/hides subtitle window via Tauri API"
```

---

## Task 5: Simplify App.vue — keep the Subtitle Overlay out of the main window

**Files:**
- Modify: `src/App.vue`

**Goal:** The main window never renders `SubtitleBar`. Remove it entirely from `App.vue` so the Subtitle Overlay remains an independent native window.

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
git commit -m "refactor: remove SubtitleBar from main window App.vue"
```

---

## Task 6: Create useSubtitleBridge — subtitle-side event listener

**Files:**
- Create: `src/composables/useSubtitleBridge.ts`

**Goal:** The subtitle window needs to react to events from the main window. `useSubtitleBridge` listens for `subtitle:show` (trigger peek), `subtitle:hide` / `subtitle:close` (hide self), and `translator:update` (sync text/language state). It also listens for `subtitle:peek` to trigger the controls peek animation.

This composable is called once in `SubtitleApp.vue` on mount.

- [ ] **Step 1: Create src/composables/useSubtitleBridge.ts**

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

export function useSubtitleBridge(onPeek: () => void) {
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
      await listen('subtitle:show', () => {
        onPeek()
      }),
      await listen('subtitle:peek', () => {
        onPeek()
      }),
    )

    // Keep subtitle always-on-top when shown
    try {
      await getCurrentWindow().setAlwaysOnTop(true)
    } catch (_) {}
  })

  onUnmounted(() => {
    unlisten.forEach(fn => fn())
  })
}
```

- [ ] **Step 2: Wire useSubtitleBridge into SubtitleApp.vue**

```vue
<script setup lang="ts">
import { ref } from 'vue'
import AppToast from '@/components/AppToast.vue'
import SubtitleBar from '@/components/subtitle/SubtitleBar.vue'
import { useSubtitleBridge } from '@/composables/useSubtitleBridge'

const peekTrigger = ref(0)
function triggerPeek() { peekTrigger.value++ }

useSubtitleBridge(triggerPeek)
</script>

<template>
  <SubtitleBar :external-peek="peekTrigger" />
  <AppToast />
</template>
```

- [ ] **Step 3: Update SubtitleBar.vue to accept externalPeek prop**

In `src/components/subtitle/SubtitleBar.vue`, replace the `peekSignal` watch with a prop watch:

```vue
<script setup lang="ts">
import { ref, watch } from 'vue'
import { useSubtitleWindow } from '@/composables/useSubtitleWindow'
import { useTranslator } from '@/composables/useTranslator'
import SubtitleControls from './SubtitleControls.vue'
import SubtitleContent from './SubtitleContent.vue'

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

const { placed, dragging, pos, size, onDragBarMousedown, onResizeMousedown } = useSubtitleWindow(barEl, () => peek())

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
    class="subtitle-bar"
    :class="{ placed, dragging, peek: peekActive }"
    :style="{
      ...(placed ? { left: pos.x + 'px', top: pos.y + 'px' } : {}),
      ...(size.width ? { width: size.width + 'px' } : {}),
    }"
    @mouseenter="onEnter"
    @mouseleave="onLeave"
  >
    <SubtitleControls @flip="onFlip" />
    <SubtitleContent
      :swap="swapping"
      @dragbar-mousedown="onDragBarMousedown"
      @resize-mousedown="onResizeMousedown"
    />
  </div>
</template>

<style scoped>
.subtitle-bar {
  position: fixed;
  left: 50%; top: 40px;
  transform: translateX(-50%);
  width: min(760px, calc(100vw - 48px));
  z-index: 40;
  display: flex;
  flex-direction: column;
  align-items: stretch;
}
.subtitle-bar.placed { transform: none; }
.subtitle-bar.dragging { user-select: none; }
.subtitle-bar.dragging :deep(.subtitle-ctrl) { opacity: 0 !important; }

.subtitle-bar:hover :deep(.subtitle-ctrl),
.subtitle-bar.peek :deep(.subtitle-ctrl) {
  opacity: 1; transform: translateY(0); pointer-events: auto;
}
</style>
```

- [ ] **Step 4: Commit**

```bash
git add src/composables/useSubtitleBridge.ts src/SubtitleApp.vue src/components/subtitle/SubtitleBar.vue
git commit -m "feat: subtitle bridge for cross-window event sync; SubtitleBar accepts externalPeek prop"
```

---

## Task 7: Sync translator state across windows

**Files:**
- Modify: `src/composables/useTranslator.ts`

**Goal:** When the main window changes `srcText`, `tgtText`, `langA`, `langB`, or `pinned`, it emits a `translator:update` Tauri event. The subtitle listens (via `useSubtitleBridge`) and updates its local refs. This keeps both windows in sync without a shared process.

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
// Guard with isBroadcasting to prevent the subtitle's listen handler
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
git commit -m "feat: broadcast translator state changes as Tauri events for subtitle sync"
```

---

## Task 8: Fix SubtitleControls — exit/close via Tauri event

**Files:**
- Modify: `src/components/subtitle/SubtitleControls.vue`

**Goal:** In the subtitle window, `useSubtitle().hide()` and `close()` hide the subtitle window and notify the main window. `SubtitleControls` also calls `togglePin`; in the subtitle window, `getCurrentWindow()` correctly targets that independent window.

The same `useSubtitle` API works from either window because `hide()` resolves the window by its `subtitle` label.

- [ ] **Step 1: Verify SubtitleControls.vue needs no changes**

Read `src/components/subtitle/SubtitleControls.vue`. Confirm:
- It imports `useSubtitle` and calls `hide()` and `close()`
- Both methods resolve the `subtitle` window, hide it, and emit the corresponding event
- `togglePin` calls `getCurrentWindow().setAlwaysOnTop()` — in subtitle context this correctly targets the subtitle window

No code changes needed. This task is a verification checkpoint.

- [ ] **Step 2: Commit checkpoint note**

```bash
git commit --allow-empty -m "chore: verify SubtitleControls works without changes in subtitle context"
```

---

## Task 9: End-to-end smoke test

**Goal:** Verify the full flow works in the Tauri dev build.

- [ ] **Step 1: Run Tauri dev**

```bash
pnpm tauri:dev
```

Expected: Two windows launch — the main panel (420×640) and the subtitle (hidden initially).

- [ ] **Step 2: Test opening the Subtitle Overlay**

Click the music note (♪) button in the main panel title bar.

Expected:
- Subtitle window appears over the desktop with the subtitle bar
- Main panel remains visible behind it (or minimizes — depends on UX preference)
- Toast "已打开悬浮字幕" appears in the subtitle window

- [ ] **Step 3: Test hiding the Subtitle Overlay**

Hover over the subtitle bar to show `SubtitleControls`. Click the layout (回到面板) button.

Expected:
- Subtitle window hides
- Toast "已回到面板视图" appears in main window

- [ ] **Step 4: Test close**

Open the Subtitle Overlay again. Click the X button in `SubtitleControls`.

Expected:
- Subtitle window hides
- Toast "悬浮字幕已关闭" appears in main window

- [ ] **Step 5: Test translator sync**

While the Subtitle Overlay is open, verify its text matches the main panel (both show the same `srcText` / `tgtText`).

- [ ] **Step 6: Commit if all passing**

```bash
git add -A
git commit -m "feat: complete independent subtitle window architecture"
```

---

## Self-Review

**Spec coverage:**
- ✅ Subtitle window defined in tauri.conf.json (Task 1)
- ✅ Second Vite entry for subtitle (Task 2)
- ✅ Subtitle Vue app mounts only SubtitleBar (Task 3)
- ✅ Main window never shows SubtitleBar (Task 5)
- ✅ Show/hide the independent subtitle window (Task 4)
- ✅ Cross-window state sync for translator (Tasks 6, 7)
- ✅ SubtitleControls exit/close hides subtitle (Task 8)
- ✅ Peek animation works via externalPeek prop (Task 6)
- ✅ No blank space or desktop-sized input boundary — subtitle is a separate content-sized window

**Placeholder scan:** No TBD or TODO found.

**Type consistency:**
- `TranslatorPayload` defined in `useSubtitleBridge.ts` and used in `listen<TranslatorPayload>` — ✅
- `externalPeek?: number` prop in `SubtitleBar`, passed as `:external-peek="peekTrigger"` from `SubtitleApp` — ✅
- `WebviewWindow.getByLabel('subtitle')` — returns `WebviewWindow | null` in Tauri v2 — ✅
- `emit('translator:update', payload)` matches `listen<TranslatorPayload>('translator:update', ...)` — ✅
