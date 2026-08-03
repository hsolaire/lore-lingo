<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useLyricMode } from '@/composables/useLyricMode'
import { useSettings } from '@/composables/useSettings'
import { useTranslator } from '@/composables/useTranslator'
import Icon from '@/components/ui/Icon.vue'

const { enter } = useLyricMode()
const { drawerOpen, toggleDrawer } = useSettings()
const { pinned, togglePin } = useTranslator()

let win: ReturnType<typeof getCurrentWindow> | null = null
try {
  win = getCurrentWindow()
} catch (_) {
  // Tauri not available (dev mode without app context)
}

const closeWin = () => win?.close()
const minimizeWin = () => win?.minimize()

function startDragging(event: MouseEvent) {
  if (!win || event.button !== 0) return
  if (event.target instanceof Element && event.target.closest('[data-window-control]')) return
  void win.startDragging().catch(() => {})
}
</script>

<template>
  <header class="titlebar" @mousedown="startDragging">
    <div class="traffic" data-window-control>
      <button type="button" class="traffic-dot r" title="关闭" aria-label="关闭窗口" @click="closeWin"></button>
      <button type="button" class="traffic-dot y" title="最小化" aria-label="最小化窗口" @click="minimizeWin"></button>
      <span class="traffic-dot g" aria-hidden="true"></span>
    </div>
    <div class="brand">
      <span class="logo" aria-hidden="true"><Icon name="search" /></span>
      <b>LensLingo</b>
      <span class="live">实时</span>
    </div>
    <span class="spacer"></span>
    <button class="icon-btn" data-window-control title="打开翻译悬浮窗" aria-label="打开翻译悬浮窗" @click="enter">
      <Icon name="music" />
    </button>
    <button
      class="icon-btn"
      data-window-control
      :class="{ active: pinned }"
      title="窗口置顶"
      aria-label="窗口置顶"
      @click="togglePin"
    >
      <Icon name="pin" />
    </button>
    <button
      class="icon-btn"
      data-window-control
      :class="{ active: drawerOpen }"
      title="设置"
      aria-label="设置"
      @click="toggleDrawer"
    >
      <Icon name="settings" />
    </button>
  </header>
</template>

<style scoped>
.titlebar {
  display: flex; align-items: center; gap: 10px;
  padding: 11px 14px;
  border-bottom: 1px solid var(--border);
  cursor: default;
  user-select: none;
}
.traffic { display: flex; gap: 8px; }
.traffic-dot {
  width: 12px; height: 12px; border: 0; border-radius: 50%; display: block; padding: 0;
}
.traffic button { cursor: pointer; }
.traffic .r { background: oklch(66% 0.17 25); }
.traffic .y { background: oklch(80% 0.13 85); }
.traffic .g { background: oklch(72% 0.15 145); cursor: default; }
.traffic button:focus-visible,
.icon-btn:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
.brand { display: flex; align-items: center; gap: 8px; margin-left: 4px; }
.brand .logo {
  width: 20px; height: 20px; border-radius: 6px;
  background: linear-gradient(135deg, var(--accent), oklch(64% 0.13 260));
  display: grid; place-items: center;
  box-shadow: 0 2px 8px oklch(72% 0.14 220 / 0.35);
}
.brand .logo :deep(svg) { width: 13px; height: 13px; }
.brand b { font: 600 13px/1 var(--font-display); letter-spacing: 0.01em; }
.brand .live {
  font: 500 10px/1 var(--mono); letter-spacing: 0.1em;
  color: var(--ok); padding: 3px 7px; border-radius: 999px;
  background: var(--ok-dim); display: inline-flex; align-items: center; gap: 5px;
}
.brand .live::before { content:""; width:5px; height:5px; border-radius:50%; background: var(--ok);
  box-shadow: 0 0 0 0 var(--ok); animation: blink 1.8s ease-in-out infinite; }
.spacer { flex: 1; }
.icon-btn {
  width: 30px; height: 30px; border-radius: 8px;
  border: 1px solid transparent; background: transparent; color: var(--muted);
  display: grid; place-items: center; cursor: pointer; transition: .15s;
}
.icon-btn:hover { background: var(--glass-hi); color: var(--fg); border-color: var(--border); }
.icon-btn.active { color: var(--accent); background: var(--accent-dim); }
.icon-btn :deep(svg) { width: 16px; height: 16px; }
</style>
