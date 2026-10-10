<script setup lang="ts">
import { useTranslator } from '@/composables/useTranslator'
import { useWindowNavigation } from '@/composables/useWindowNavigation'
import { currentWindowClient } from '@/services/windowClient'
import Icon from '@/components/ui/Icon.vue'
import type { PlatformKind } from '@/types/desktop'

withDefaults(defineProps<{ platform?: PlatformKind }>(), { platform: 'unsupported' })

const { openSettings, openSubtitle } = useWindowNavigation('main')
const { pinned, togglePin } = useTranslator()

const closeWin = () => void currentWindowClient.close().catch(() => {})
const minimizeWin = () => void currentWindowClient.minimize().catch(() => {})
const toggleMaximizeWin = () => void currentWindowClient.toggleMaximize().catch(() => {})

function startDragging(event: MouseEvent) {
  if (event.button !== 0) return
  if (event.target instanceof Element && event.target.closest('[data-window-control]')) return
  void currentWindowClient.startDragging().catch(() => {})
}
</script>

<template>
  <header class="titlebar" :class="{ 'native-controls': platform === 'macos' }" @mousedown="startDragging">
    <div v-if="platform !== 'macos'" class="traffic" data-window-control>
      <button type="button" class="traffic-control close" title="关闭" aria-label="关闭窗口" @click="closeWin">
        <span class="traffic-dot"><Icon name="x" /></span>
      </button>
      <button type="button" class="traffic-control minimize" title="最小化" aria-label="最小化窗口" @click="minimizeWin">
        <span class="traffic-dot"><Icon name="minus" /></span>
      </button>
      <button
        type="button"
        class="traffic-control maximize"
        title="最大化或还原"
        aria-label="最大化或还原窗口"
        @click="toggleMaximizeWin"
      >
        <span class="traffic-dot"><Icon name="maximize" /></span>
      </button>
    </div>
    <div class="brand">
      <span class="logo" aria-hidden="true"><Icon name="search" /></span>
      <b>LoreLingo</b>
      <span class="live">实时</span>
    </div>
    <span class="spacer"></span>
    <button class="icon-btn" data-window-control title="打开悬浮字幕" aria-label="打开悬浮字幕" @click="openSubtitle">
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
      title="打开设置"
      aria-label="设置"
      @click="openSettings"
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
.traffic { display: flex; align-items: center; margin-left: -6px; }
/* 原生交通灯位于 WebView 上方，标题栏预留左侧空间避免遮挡。 */
.titlebar.native-controls { padding-left: 88px; }
.traffic-control {
  width: 23px; height: 30px; border: 0; padding: 0;
  display: grid; place-items: center;
  background: transparent; cursor: pointer;
}
.traffic-dot {
  width: 14px; height: 14px; border-radius: 50%;
  display: grid; place-items: center;
  border: 0.5px solid transparent;
  transition: filter 120ms ease, transform 120ms ease;
}
.traffic-control.close .traffic-dot { background: #ff5f57; border-color: #e0443e; color: #7d0000; }
.traffic-control.minimize .traffic-dot { background: #febc2e; border-color: #dea123; color: #995700; }
.traffic-control.maximize .traffic-dot { background: #28c840; border-color: #1aab29; color: #006500; }
.traffic-dot :deep(svg) {
  width: 8px; height: 8px; opacity: 0;
  stroke-width: 3.2;
  transition: opacity 100ms ease;
}
.traffic-control.maximize .traffic-dot :deep(svg) { width: 10px; height: 10px; }
.traffic:hover .traffic-dot :deep(svg),
.traffic:focus-within .traffic-dot :deep(svg) { opacity: 1; }
.traffic-control:hover .traffic-dot { filter: brightness(1.06) saturate(1.05); }
.traffic-control:active .traffic-dot { filter: brightness(0.84); transform: scale(0.92); }
.traffic-control:focus-visible,
.icon-btn:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
@media (prefers-reduced-motion: reduce) {
  .traffic-dot, .traffic-dot :deep(svg) { transition: none; }
}
.brand { display: flex; align-items: center; gap: 8px; margin-left: -3px; }
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
