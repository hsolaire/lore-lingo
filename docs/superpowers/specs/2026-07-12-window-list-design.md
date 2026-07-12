# Window List Feature — Design Spec

**Date:** 2026-07-12  
**Scope:** Windows only (macOS deferred)  
**Status:** Approved

---

## Overview

When the user clicks the CaptureTarget card in "捕获应用" mode, a dropdown list appears showing all currently visible windows (with titles). The user selects one; the card updates to show the selected window's icon, title, and exe name. This replaces the current hardcoded "VLC" placeholder.

---

## Architecture & Data Flow

```
[User clicks CaptureTarget card]
       ↓
[Frontend: invoke('list_windows')]
       ↓
[Rust: EnumWindows → filter no-title windows
       → GetWindowThreadProcessId → OpenProcess
       → QueryFullProcessImageName → exe path
       → SHGetFileInfo → HICON → PNG bytes → base64]
       ↓
[Returns Vec<WindowInfo>]
  { id: u32, title: String, exe: String, icon_b64: Option<String> }
       ↓
[Frontend: renders dropdown list]
  - Each row: icon (img or fallback letter) + title + exe name
  - Refresh button at top triggers re-invoke
  - Loading spinner while fetching
       ↓
[User clicks a row → selectedWindow ref updated, dropdown closes]
       ↓
[CaptureTarget card shows: icon + title + exe name]
```

---

## Backend (Rust)

**Dependency:** `windows` crate (Microsoft official, feature-gated to Win32 subset needed).

**New Tauri command:** `list_windows() -> Vec<WindowInfo>`

**WindowInfo struct:**
```rust
struct WindowInfo {
    id: u32,        // HWND as u32
    title: String,
    exe: String,    // basename only, e.g. "vlc.exe"
    icon_b64: Option<String>,  // PNG base64, None if unavailable
}
```

**Window enumeration logic:**
1. `EnumWindows` to collect all top-level HWNDs
2. Filter: `IsWindowVisible` = true AND `GetWindowTextW` length > 0
3. For each passing window: `GetWindowThreadProcessId` → `OpenProcess` → `QueryFullProcessImageNameW` → extract exe basename
4. Icon: `SHGetFileInfoW` with `SHGFI_ICON | SHGFI_SMALLICON` on the exe path → `HICON` → `GetIconInfo` → `CreateCompatibleDC` → render to DIB → encode PNG → base64
5. On any icon failure: return `icon_b64: None` (frontend falls back to first letter of exe name)

**Register** in `invoke_handler`: add `list_windows` alongside existing `capture_region`, `list_screens`.

---

## Frontend

**`useCaptureSource.ts` additions:**
```ts
interface WindowInfo {
  id: number
  title: string
  exe: string
  icon_b64: string | null
}

const windowList = ref<WindowInfo[]>([])
const selectedWindow = ref<WindowInfo | null>(null)
const listOpen = ref(false)
const listLoading = ref(false)

async function fetchWindows() { ... }   // invoke('list_windows')
function selectWindow(w: WindowInfo) { ... }
function toggleList() { ... }
```

`targetName` and `targetMeta` become derived from `selectedWindow` when set, otherwise show "未选择窗口" / "点击下方选择目标窗口".

**`CaptureTarget.vue` changes:**
- Card becomes clickable → calls `toggleList()`
- Below the card: `v-if="listOpen"` dropdown panel
  - Spinner while `listLoading`
  - Refresh icon button (top-right of panel) → calls `fetchWindows()`
  - List of rows: `[icon/letter] [title] [exe]`
  - Clicking a row → `selectWindow(w)`, list closes

**Icon rendering per row:**
```html
<img v-if="w.icon_b64" :src="`data:image/png;base64,${w.icon_b64}`" />
<span v-else class="fallback-letter">{{ w.exe[0].toUpperCase() }}</span>
```

---

## Filtering Rules

Only windows passing **both** conditions are listed:
- `IsWindowVisible() == true`
- `GetWindowTextW()` returns non-empty string

No additional filtering (system windows, tray, etc. are included if they have a visible title).

---

## Error Handling

- `list_windows` returns empty `Vec` (not an error) if `EnumWindows` fails
- Individual windows that fail icon extraction still appear with `icon_b64: null`
- Frontend shows "未找到可捕获的窗口" when list is empty (matches current UI text)
- No retry on error — user can manually hit the refresh button

---

## Out of Scope

- macOS support (deferred)
- Window content thumbnails (only exe icon)
- Filtering by window type beyond visible+titled
- Auto-refresh / watching for new windows
