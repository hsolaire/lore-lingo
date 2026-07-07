<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useLyricMode } from '@/composables/useLyricMode'
import { useSettings } from '@/composables/useSettings'
import { useTranslator } from '@/composables/useTranslator'
import Icon from '@/components/ui/Icon.vue'

const { enter } = useLyricMode()
const { drawerOpen, toggleDrawer } = useSettings()
const { pinned, togglePin } = useTranslator()

const win = getCurrentWindow()
const closeWin   = () => win.close()
const minimizeWin = () => win.minimize()
</script>

<template>
  <header class="titlebar">
    <div class="traffic">
      <i class="r" title="关闭" @click="closeWin"></i>
      <i class="y" title="最小化" @click="minimizeWin"></i>
      <i class="g"></i>
    </div>
    <div class="brand">
      <span class="logo" aria-hidden="true"><Icon name="search" /></span>
      <b>LensLingo</b>
      <span class="live">实时</span>
    </div>
    <span class="spacer"></span>
    <button class="icon-btn" title="切换为屏幕歌词条" aria-label="切换为屏幕歌词条" @click="enter">
      <Icon name="music" />
    </button>
    <button
      class="icon-btn"
      :class="{ active: pinned }"
      title="窗口置顶"
      aria-label="窗口置顶"
      @click="togglePin"
    >
      <Icon name="pin" />
    </button>
    <button
      class="icon-btn"
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
  -webkit-app-region: drag;
  border-bottom: 1px solid var(--border);
}
.traffic { display: flex; gap: 8px; -webkit-app-region: no-drag; }
.traffic i { width: 12px; height: 12px; border-radius: 50%; display: block; cursor: pointer; }
.traffic .r { background: oklch(66% 0.17 25); }
.traffic .y { background: oklch(80% 0.13 85); }
.traffic .g { background: oklch(72% 0.15 145); cursor: default; }
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
  -webkit-app-region: no-drag;
  width: 30px; height: 30px; border-radius: 8px;
  border: 1px solid transparent; background: transparent; color: var(--muted);
  display: grid; place-items: center; cursor: pointer; transition: .15s;
}
.icon-btn:hover { background: var(--glass-hi); color: var(--fg); border-color: var(--border); }
.icon-btn.active { color: var(--accent); background: var(--accent-dim); }
.icon-btn :deep(svg) { width: 16px; height: 16px; }
</style>
