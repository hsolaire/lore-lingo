# Window List Feature Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the hardcoded "VLC" placeholder in CaptureTarget with a real dropdown that enumerates visible Windows windows, shows their exe icons, and lets the user select one.

**Architecture:** A new `list_windows` Tauri command (Rust, Windows-only) uses `EnumWindows` + `SHGetFileInfoW` to return `Vec<WindowInfo>` with title, exe basename, and PNG icon as base64. The frontend composable `useCaptureSource.ts` gains four new refs (`windowList`, `selectedWindow`, `listOpen`, `listLoading`) and three functions (`fetchWindows`, `selectWindow`, `toggleList`). `CaptureTarget.vue` becomes a clickable card that expands into the dropdown below it.

**Tech Stack:** Rust (`windows` crate 0.58, `image` crate for PNG encoding), Vue 3 + TypeScript, Tauri 2 invoke API.

## Global Constraints

- Windows-only backend code must be gated with `#[cfg(target_os = "windows")]`
- `windows` crate features: only the exact Win32 subsets needed (see Task 1)
- Icon size: 32×32 px (SHGFI_LARGEICON)
- PNG encode via `image` crate (already transitively available via `screenshots`)
- All Chinese UI strings must match existing style: `未选择窗口`, `点击下方选择目标窗口`, `未找到可捕获的窗口`
- No new composables — extend `useCaptureSource.ts` only
- Tauri command registered in the existing `invoke_handler![]` macro in `lib.rs`

---

### Task 1: Add `windows` crate dependency and `list_windows` Rust command (skeleton)

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Produces: `list_windows()` Tauri command returning `Vec<WindowInfo>` (initially returns empty vec — real impl in Task 2)
- Produces: `WindowInfo` struct with fields `id: u32`, `title: String`, `exe: String`, `icon_b64: Option<String>`

- [ ] **Step 1: Add `windows` crate to Cargo.toml**

Open `src-tauri/Cargo.toml` and add under `[dependencies]`:

```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.58", features = [
  "Win32_Foundation",
  "Win32_UI_WindowsAndMessaging",
  "Win32_System_Threading",
  "Win32_UI_Shell",
  "Win32_Graphics_Gdi",
] }
```

- [ ] **Step 2: Add WindowInfo struct and skeleton command to lib.rs**

Add after the existing `use` imports at the top of `src-tauri/src/lib.rs`:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct WindowInfo {
    id: u32,
    title: String,
    exe: String,
    icon_b64: Option<String>,
}

#[command]
fn list_windows() -> Vec<WindowInfo> {
    vec![]
}
```

- [ ] **Step 3: Register list_windows in invoke_handler**

In `lib.rs`, change the existing invoke_handler line from:
```rust
.invoke_handler(tauri::generate_handler![capture_region, list_screens])
```
to:
```rust
.invoke_handler(tauri::generate_handler![capture_region, list_screens, list_windows])
```

- [ ] **Step 4: Verify it compiles**

```bash
cd src-tauri
cargo check
```

Expected: no errors. Warnings about unused imports are OK.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/lib.rs
git commit -m "feat(rust): add WindowInfo struct and list_windows skeleton"
```

---

### Task 2: Implement window enumeration in list_windows

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: `WindowInfo` struct from Task 1, `windows` crate features from Task 1
- Produces: `list_windows()` now returns real data — visible top-level windows with non-empty titles, exe basename, icon_b64 always None (icon impl in Task 3)

- [ ] **Step 1: Add Windows-only imports to lib.rs**

Add at the top of `src-tauri/src/lib.rs`, after existing imports:

```rust
#[cfg(target_os = "windows")]
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{CloseHandle, BOOL, HWND, LPARAM},
        System::Threading::{
            OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
            PROCESS_QUERY_LIMITED_INFORMATION,
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
            IsWindowVisible,
        },
    },
};
```

- [ ] **Step 2: Replace list_windows body with real enumeration**

Replace the entire `list_windows` function with:

```rust
#[command]
fn list_windows() -> Vec<WindowInfo> {
    #[cfg(not(target_os = "windows"))]
    return vec![];

    #[cfg(target_os = "windows")]
    {
        let results: std::sync::Mutex<Vec<WindowInfo>> = std::sync::Mutex::new(Vec::new());
        let results_ptr = &results as *const _ as isize;

        unsafe {
            let _ = EnumWindows(
                Some(enum_windows_callback),
                LPARAM(results_ptr),
            );
        }

        results.into_inner().unwrap_or_default()
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    use windows::Win32::Foundation::TRUE;

    // Skip invisible windows
    if !IsWindowVisible(hwnd).as_bool() {
        return TRUE;
    }

    // Skip windows with no title
    let title_len = GetWindowTextLengthW(hwnd);
    if title_len == 0 {
        return TRUE;
    }

    let mut title_buf = vec![0u16; (title_len + 1) as usize];
    GetWindowTextW(hwnd, &mut title_buf);
    let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

    // Get process id
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));

    // Get exe path
    let exe = get_exe_name(pid).unwrap_or_else(|| "unknown.exe".to_string());

    let results = &*(lparam.0 as *const std::sync::Mutex<Vec<WindowInfo>>);
    if let Ok(mut list) = results.lock() {
        list.push(WindowInfo {
            id: hwnd.0 as u32,
            title,
            exe,
            icon_b64: None, // filled in Task 3
        });
    }

    TRUE
}

#[cfg(target_os = "windows")]
fn get_exe_name(pid: u32) -> Option<String> {
    use std::path::Path;
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = vec![0u16; 1024];
        let mut len = buf.len() as u32;
        QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len).ok()?;
        let _ = CloseHandle(handle);
        let path = String::from_utf16_lossy(&buf[..len as usize]);
        Some(Path::new(&path).file_name()?.to_string_lossy().to_string())
    }
}
```

- [ ] **Step 3: Verify it compiles**

```bash
cd src-tauri
cargo check
```

Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat(rust): implement window enumeration in list_windows"
```

---

### Task 3: Implement exe icon extraction (PNG base64)

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: `get_exe_name` helper and `WindowInfo` from Task 2; `windows` crate `Win32_UI_Shell` and `Win32_Graphics_Gdi` features from Task 1
- Produces: `icon_b64` field in `WindowInfo` is now populated when icon extraction succeeds

- [ ] **Step 1: Add image crate to Cargo.toml**

The `screenshots` crate already brings in `image` transitively, but add it explicitly for PNG encoding. Add under `[dependencies]` in `src-tauri/Cargo.toml`:

```toml
image = { version = "0.25", default-features = false, features = ["png"] }
```

> **Note:** If `cargo check` reports a version conflict with `screenshots`' transitive `image` dep, check the version `screenshots 0.8` pulls in with `cargo tree -i image` and match that version number here.

- [ ] **Step 2: Add icon extraction imports**

Add to the Windows-only import block at the top of `lib.rs`:

```rust
#[cfg(target_os = "windows")]
use windows::Win32::{
    UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON},
    Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW,
        SelectObject, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    },
    UI::WindowsAndMessaging::DestroyIcon,
};
```

- [ ] **Step 3: Add get_exe_icon helper function**

Add this function below `get_exe_name` in `lib.rs`:

```rust
#[cfg(target_os = "windows")]
fn get_exe_icon_b64(exe_path: &str) -> Option<String> {
    use std::io::Cursor;
    use image::{ImageBuffer, RgbaImage};

    unsafe {
        let wide: Vec<u16> = exe_path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut shfi: SHFILEINFOW = std::mem::zeroed();
        let ret = SHGetFileInfoW(
            windows::core::PCWSTR(wide.as_ptr()),
            Default::default(),
            Some(&mut shfi),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );
        if ret == 0 || shfi.hIcon.is_invalid() {
            return None;
        }

        // Get bitmap info from the icon
        let mut bmp: BITMAP = std::mem::zeroed();
        let mut bmi: BITMAPINFO = std::mem::zeroed();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = 32;
        bmi.bmiHeader.biHeight = -32; // top-down
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB.0;

        let dc = CreateCompatibleDC(None);
        let mut pixels: Vec<u8> = vec![0u8; 32 * 32 * 4];

        // Draw icon into a compatible bitmap
        let hbmp = windows::Win32::Graphics::Gdi::CreateCompatibleBitmap(dc, 32, 32);
        let old = SelectObject(dc, hbmp);
        windows::Win32::UI::WindowsAndMessaging::DrawIconEx(
            dc,
            0, 0,
            shfi.hIcon,
            32, 32,
            0,
            None,
            windows::Win32::UI::WindowsAndMessaging::DI_NORMAL,
        ).ok()?;
        GetDIBits(dc, hbmp, 0, 32, Some(pixels.as_mut_ptr() as *mut _), &mut bmi, DIB_RGB_COLORS);
        SelectObject(dc, old);
        DeleteObject(hbmp);
        DeleteDC(dc);
        let _ = DestroyIcon(shfi.hIcon);

        // Windows DIB is BGRA — convert to RGBA
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // B <-> R
        }

        let img: RgbaImage = ImageBuffer::from_raw(32, 32, pixels)?;
        let mut buf = Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Png).ok()?;
        Some(base64::engine::general_purpose::STANDARD.encode(buf.into_inner()))
    }
}
```

- [ ] **Step 4: Wire get_exe_icon_b64 into enum_windows_callback**

In `enum_windows_callback`, change the `icon_b64: None` line. First, we need the full exe path (not just basename) for `SHGetFileInfoW`. Modify `get_exe_name` to also return the full path, then extract the basename separately.

Replace the existing `get_exe_name` function with:

```rust
#[cfg(target_os = "windows")]
fn get_exe_info(pid: u32) -> Option<(String, String)> {
    // Returns (full_path, basename)
    use std::path::Path;
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = vec![0u16; 1024];
        let mut len = buf.len() as u32;
        QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len).ok()?;
        let _ = CloseHandle(handle);
        let full_path = String::from_utf16_lossy(&buf[..len as usize]);
        let basename = Path::new(&full_path).file_name()?.to_string_lossy().to_string();
        Some((full_path, basename))
    }
}
```

Then update `enum_windows_callback` to use `get_exe_info`:

```rust
    // Replace: let exe = get_exe_name(pid).unwrap_or_else(|| "unknown.exe".to_string());
    // With:
    let (exe_path, exe) = get_exe_info(pid)
        .unwrap_or_else(|| ("".to_string(), "unknown.exe".to_string()));
    let icon_b64 = if exe_path.is_empty() { None } else { get_exe_icon_b64(&exe_path) };

    // And in WindowInfo construction:
    list.push(WindowInfo {
        id: hwnd.0 as u32,
        title,
        exe,
        icon_b64,
    });
```

- [ ] **Step 5: Verify it compiles**

```bash
cd src-tauri
cargo check
```

Expected: no errors.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/lib.rs
git commit -m "feat(rust): extract exe icon as PNG base64 in list_windows"
```

---

### Task 4: Frontend composable — window list state and actions

**Files:**
- Modify: `src/composables/useCaptureSource.ts`

**Interfaces:**
- Produces (for Task 5):
  - `WindowInfo` interface: `{ id: number; title: string; exe: string; icon_b64: string | null }`
  - `windowList: Ref<WindowInfo[]>`
  - `selectedWindow: Ref<WindowInfo | null>`
  - `listOpen: Ref<boolean>`
  - `listLoading: Ref<boolean>`
  - `fetchWindows(): Promise<void>`
  - `selectWindow(w: WindowInfo): void`
  - `toggleList(): void`
  - `targetName: ComputedRef<string>` — now returns `selectedWindow.value?.title ?? '未选择窗口'` when `sourceMode === 'app'`
  - `targetMeta: ComputedRef<string>` — now returns `selectedWindow.value?.exe ?? '点击下方选择目标窗口'` when `sourceMode === 'app'`

- [ ] **Step 1: Replace useCaptureSource.ts entirely**

```typescript
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './useToast'

export type SourceMode = 'app' | 'region'

export interface WindowInfo {
  id: number
  title: string
  exe: string
  icon_b64: string | null
}

const sourceMode = ref<SourceMode>('app')
const selecting = ref(false)
const windowList = ref<WindowInfo[]>([])
const selectedWindow = ref<WindowInfo | null>(null)
const listOpen = ref(false)
const listLoading = ref(false)
let selectTimer: ReturnType<typeof setTimeout> | undefined

export function useCaptureSource() {
  const { toast } = useToast()

  const targetName = computed(() => {
    if (sourceMode.value === 'app') {
      return selectedWindow.value?.title ?? '未选择窗口'
    }
    return '自定义区域'
  })

  const targetMeta = computed(() => {
    if (sourceMode.value === 'app') {
      return selectedWindow.value?.exe ?? '点击下方选择目标窗口'
    }
    return '520 × 118 · 屏幕坐标 (700, 46)'
  })

  async function fetchWindows() {
    listLoading.value = true
    try {
      windowList.value = await invoke<WindowInfo[]>('list_windows')
    } catch {
      windowList.value = []
    } finally {
      listLoading.value = false
    }
  }

  function selectWindow(w: WindowInfo) {
    selectedWindow.value = w
    listOpen.value = false
  }

  function toggleList() {
    if (!listOpen.value) {
      listOpen.value = true
      fetchWindows()
    } else {
      listOpen.value = false
    }
  }

  function startSelect() {
    selecting.value = true
    toast('拖拽屏幕以框选区域…')
    clearTimeout(selectTimer)
    selectTimer = setTimeout(() => {
      selecting.value = false
      toast('区域已捕获')
    }, 1900)
  }

  function setSource(mode: SourceMode) {
    sourceMode.value = mode
    if (mode === 'region') {
      listOpen.value = false
      startSelect()
    }
  }

  function reselect() {
    if (sourceMode.value === 'region') startSelect()
    else toggleList()
  }

  return {
    sourceMode,
    selecting,
    windowList,
    selectedWindow,
    listOpen,
    listLoading,
    targetName,
    targetMeta,
    fetchWindows,
    selectWindow,
    toggleList,
    setSource,
    reselect,
  }
}
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
npx vue-tsc --noEmit
```

Expected: no errors in `useCaptureSource.ts`.

- [ ] **Step 3: Commit**

```bash
git add src/composables/useCaptureSource.ts
git commit -m "feat(composable): add window list state and actions to useCaptureSource"
```

---

### Task 5: Update CaptureTarget.vue to show dropdown

**Files:**
- Modify: `src/components/main-panel/CaptureTarget.vue`

**Interfaces:**
- Consumes from Task 4: `windowList`, `selectedWindow`, `listOpen`, `listLoading`, `fetchWindows`, `selectWindow`, `toggleList`, `targetName`, `targetMeta`

- [ ] **Step 1: Replace CaptureTarget.vue entirely**

```vue
<script setup lang="ts">
import { useCaptureSource } from '@/composables/useCaptureSource'

const {
  targetName,
  targetMeta,
  selectedWindow,
  windowList,
  listOpen,
  listLoading,
  toggleList,
  selectWindow,
  fetchWindows,
} = useCaptureSource()

function iconSrc(b64: string) {
  return `data:image/png;base64,${b64}`
}

function fallbackLetter(exe: string) {
  return exe.charAt(0).toUpperCase()
}
</script>

<template>
  <div class="capture-target-wrap">
    <!-- Selected window card (clickable) -->
    <div class="target" :class="{ open: listOpen }" @click="toggleList" role="button" tabindex="0" @keydown.enter="toggleList">
      <div class="thumb" v-if="selectedWindow && selectedWindow.icon_b64">
        <img :src="iconSrc(selectedWindow.icon_b64)" width="26" height="26" style="border-radius:6px;display:block" />
      </div>
      <div class="thumb letter" v-else>
        {{ selectedWindow ? fallbackLetter(selectedWindow.exe) : '未' }}
      </div>
      <div style="flex:1;min-width:0">
        <b>{{ targetName }}</b>
        <div class="dim">{{ targetMeta }}</div>
      </div>
      <div class="chevron" :class="{ up: listOpen }">›</div>
    </div>

    <!-- Dropdown list -->
    <div class="win-list" v-if="listOpen">
      <div class="list-header">
        <span class="list-title">选择窗口</span>
        <button class="refresh-btn" @click.stop="fetchWindows" :disabled="listLoading" title="刷新">
          <svg v-if="!listLoading" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M23 4v6h-6M1 20v-6h6"/>
            <path d="M3.51 9a9 9 0 0114.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0020.49 15"/>
          </svg>
          <svg v-else width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="spin">
            <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
          </svg>
        </button>
      </div>

      <div class="list-empty" v-if="!listLoading && windowList.length === 0">
        未找到可捕获的窗口
      </div>

      <div
        v-for="w in windowList"
        :key="w.id"
        class="win-row"
        :class="{ selected: selectedWindow?.id === w.id }"
        @click="selectWindow(w)"
      >
        <div class="win-icon">
          <img v-if="w.icon_b64" :src="iconSrc(w.icon_b64)" width="20" height="20" style="display:block" />
          <span v-else class="fallback-letter">{{ fallbackLetter(w.exe) }}</span>
        </div>
        <div class="win-info">
          <div class="win-title">{{ w.title }}</div>
          <div class="win-exe">{{ w.exe }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.capture-target-wrap {
  margin: 8px 14px 0;
}

.target {
  padding: 8px 11px;
  display: flex; align-items: center; gap: 9px;
  background: oklch(20% 0.02 260 / 0.4);
  border: 1px solid var(--border); border-radius: 10px;
  font-size: 12px; color: var(--muted);
  cursor: pointer; user-select: none;
  transition: border-color .15s;
}
.target:hover, .target.open { border-color: var(--accent); }
.target b { color: var(--fg); font-weight: 550; }
.dim { color: var(--faint); font: 400 11px/1 var(--mono); }

.thumb {
  width: 26px; height: 26px; border-radius: 6px; flex: none;
  background: linear-gradient(135deg, oklch(55% 0.11 25), oklch(48% 0.09 300));
  display: grid; place-items: center; font: 600 12px/1 var(--font); color: #fff;
  overflow: hidden;
}
.thumb.letter { background: linear-gradient(135deg, oklch(55% 0.11 25), oklch(48% 0.09 300)); }

.chevron {
  color: var(--faint); font-size: 14px; line-height: 1;
  transform: rotate(90deg); transition: transform .2s;
  margin-left: auto; flex: none;
}
.chevron.up { transform: rotate(-90deg); }

/* Dropdown */
.win-list {
  margin-top: 4px;
  background: oklch(18% 0.02 260 / 0.95);
  border: 1px solid var(--border); border-radius: 10px;
  overflow: hidden; max-height: 240px; overflow-y: auto;
}

.list-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 8px 10px 6px; border-bottom: 1px solid var(--border);
}
.list-title { font: 500 11px/1 var(--font); color: var(--muted); }
.refresh-btn {
  background: none; border: none; cursor: pointer; color: var(--muted);
  padding: 2px; border-radius: 4px; display: grid; place-items: center;
  transition: color .15s;
}
.refresh-btn:hover { color: var(--accent); }
.refresh-btn:disabled { opacity: 0.4; cursor: default; }

.list-empty {
  padding: 16px 12px; text-align: center;
  font: 400 11px/1 var(--font); color: var(--faint);
}

.win-row {
  display: flex; align-items: center; gap: 8px;
  padding: 7px 10px; cursor: pointer; transition: background .12s;
}
.win-row:hover { background: oklch(26% 0.03 260 / 0.6); }
.win-row.selected { background: oklch(30% 0.05 260 / 0.5); }

.win-icon {
  width: 20px; height: 20px; flex: none;
  display: grid; place-items: center;
}
.fallback-letter {
  width: 20px; height: 20px; border-radius: 4px;
  background: linear-gradient(135deg, oklch(50% 0.1 220), oklch(44% 0.08 280));
  display: grid; place-items: center;
  font: 600 10px/1 var(--font); color: #fff;
}

.win-info { flex: 1; min-width: 0; }
.win-title {
  font: 500 12px/1.3 var(--font); color: var(--fg);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.win-exe { font: 400 10px/1 var(--mono); color: var(--faint); margin-top: 2px; }

@keyframes spin { to { transform: rotate(360deg); } }
.spin { animation: spin .7s linear infinite; }
</style>
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
npx vue-tsc --noEmit
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src/components/main-panel/CaptureTarget.vue
git commit -m "feat(ui): implement window dropdown in CaptureTarget"
```

---

### Task 6: End-to-end smoke test (manual)

**Files:** none modified

- [ ] **Step 1: Build and run the app**

```bash
npm run tauri dev
```

- [ ] **Step 2: Click the CaptureTarget card**

Expected:
- Dropdown opens below the card
- Loading spinner appears briefly
- A list of visible windows with titles appears
- Each row shows exe icon (or fallback letter), window title, and exe name

- [ ] **Step 3: Select a window**

Click any row in the list.

Expected:
- Dropdown closes
- CaptureTarget card now shows the selected window's icon + title (top line) and exe name (bottom line)

- [ ] **Step 4: Test refresh button**

Re-open the dropdown, click the refresh icon.

Expected: list reloads (spinner shows briefly, list repopulates).

- [ ] **Step 5: Test empty state**

Temporarily modify `list_windows` in Rust to return `vec![]`, rebuild, and open dropdown.

Expected: "未找到可捕获的窗口" message appears.

Revert the temporary change after verifying.

- [ ] **Step 6: Commit if any fixes were made**

```bash
git add -p
git commit -m "fix: address issues found in smoke test"
```
