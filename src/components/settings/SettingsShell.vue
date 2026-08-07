<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, type DeepReadonly } from 'vue'
import { Captions, House, Keyboard, Languages, ScanLine, Search, X } from 'lucide-vue-next'
import { NButton, NIcon } from 'naive-ui'

import { useWindowNavigation } from '@/composables/useWindowNavigation'
import { currentWindowClient } from '@/services/windowClient'
import type { BootstrapSnapshot } from '@/types/app'
import OcrSettingsPanel from './OcrSettingsPanel.vue'
import ShortcutBindingsPanel from './ShortcutBindingsPanel.vue'
import SubtitleLibraryPanel from './SubtitleLibraryPanel.vue'
import TranslationSettingsPanel from './TranslationSettingsPanel.vue'

const props = defineProps<{ snapshot?: DeepReadonly<BootstrapSnapshot> | null }>()

type SettingsTab = 'translation' | 'shortcuts' | 'ocr' | 'subtitles'

const activeTab = ref<SettingsTab>('translation')
const { openMain, hideCurrent } = useWindowNavigation('settings')

const tabs = [
  { id: 'translation' as const, label: '翻译', icon: Languages },
  { id: 'shortcuts' as const, label: '快捷键', icon: Keyboard },
  { id: 'ocr' as const, label: 'OCR', icon: ScanLine },
  { id: 'subtitles' as const, label: '字幕库', icon: Captions },
]

const activePanelId = computed(() => `settings-panel-${activeTab.value}`)

function activateTab(tab: SettingsTab) {
  activeTab.value = tab
}

function moveTab(direction: 1 | -1) {
  const currentIndex = tabs.findIndex((tab) => tab.id === activeTab.value)
  const nextIndex = (currentIndex + direction + tabs.length) % tabs.length
  activeTab.value = tabs[nextIndex].id
  requestAnimationFrame(() => document.getElementById(`settings-tab-${activeTab.value}`)?.focus())
}

function onTabKeydown(event: KeyboardEvent) {
  if (event.key === 'ArrowDown' || event.key === 'ArrowRight') {
    event.preventDefault()
    moveTab(1)
  } else if (event.key === 'ArrowUp' || event.key === 'ArrowLeft') {
    event.preventDefault()
    moveTab(-1)
  }
}

function onWindowKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') void hideCurrent()
}

// 步骤 1. 设置窗口挂载后监听 Escape，让键盘用户能够隐藏独立窗口。
onMounted(() => window.addEventListener('keydown', onWindowKeydown))

// 步骤 2. 设置窗口卸载时移除监听，避免重复打开后累积事件处理器。
onUnmounted(() => window.removeEventListener('keydown', onWindowKeydown))

function startDragging(event: MouseEvent) {
  if (event.button !== 0) return
  if (event.target instanceof Element && event.target.closest('[data-window-control]')) return
  void currentWindowClient.startDragging()
}
</script>

<template>
  <main class="settings-window">
    <header class="settings-titlebar" @mousedown="startDragging">
      <button
        type="button"
        class="close-control"
        data-window-control
        aria-label="关闭设置"
        title="关闭设置"
        @click="hideCurrent"
      >
        <X :size="8" :stroke-width="3" />
      </button>
      <div class="settings-brand">
        <span class="brand-icon" aria-hidden="true"><Search :size="14" /></span>
        <b>设置</b>
        <span>LoreLingo</span>
      </div>
      <span class="title-spacer" />
      <NButton
        quaternary
        circle
        data-window-control
        aria-label="打开主面板"
        title="打开主面板"
        @click="openMain"
      >
        <template #icon><NIcon :component="House" /></template>
      </NButton>
    </header>

    <div class="settings-layout">
      <nav class="settings-nav" role="tablist" aria-label="设置分类" @keydown="onTabKeydown">
        <span class="nav-title">设置</span>
        <button
          v-for="tab in tabs"
          :id="`settings-tab-${tab.id}`"
          :key="tab.id"
          type="button"
          role="tab"
          class="nav-item"
          :class="{ active: activeTab === tab.id }"
          :tabindex="activeTab === tab.id ? 0 : -1"
          :aria-selected="activeTab === tab.id"
          :aria-controls="`settings-panel-${tab.id}`"
          @click="activateTab(tab.id)"
        >
          <span class="nav-icon" aria-hidden="true">
            <NIcon :component="tab.icon" size="16" />
          </span>
          <span class="nav-label">{{ tab.label }}</span>
        </button>
      </nav>

      <section class="settings-content" :aria-owns="activePanelId">
        <div
          id="settings-panel-translation"
          role="tabpanel"
          aria-labelledby="settings-tab-translation"
          :hidden="activeTab !== 'translation'"
        >
          <TranslationSettingsPanel
            :settings="props.snapshot?.settings.translation ?? null"
            :default-model="props.snapshot?.defaultModel ?? null"
          />
        </div>
        <div
          id="settings-panel-shortcuts"
          role="tabpanel"
          aria-labelledby="settings-tab-shortcuts"
          :hidden="activeTab !== 'shortcuts'"
        >
          <ShortcutBindingsPanel
            :shortcuts="props.snapshot?.shortcuts ?? []"
            :available="props.snapshot?.platform.globalShortcuts ?? false"
            :platform="props.snapshot?.platform.platform ?? 'unsupported'"
          />
        </div>
        <div
          id="settings-panel-ocr"
          role="tabpanel"
          aria-labelledby="settings-tab-ocr"
          :hidden="activeTab !== 'ocr'"
        >
          <OcrSettingsPanel
            :settings="props.snapshot?.settings.ocr ?? null"
            :system-available="props.snapshot?.platform.systemOcr ?? false"
            :platform="props.snapshot?.platform.platform ?? 'unsupported'"
          />
        </div>
        <div
          id="settings-panel-subtitles"
          role="tabpanel"
          aria-labelledby="settings-tab-subtitles"
          :hidden="activeTab !== 'subtitles'"
        >
          <SubtitleLibraryPanel />
        </div>
      </section>
    </div>
  </main>
</template>

<style scoped>
.settings-window {
  width: 100%; height: 100%; min-width: 0; min-height: 0;
  display: flex; flex-direction: column; overflow: hidden;
  border: 1px solid var(--border-hi); border-radius: var(--radius);
  background: var(--glass); backdrop-filter: blur(40px) saturate(1.65);
  -webkit-backdrop-filter: blur(40px) saturate(1.65);
  box-shadow: var(--shadow); isolation: isolate;
}
.settings-titlebar { display: flex; align-items: center; gap: 10px; min-height: 51px; padding: 9px 13px; border-bottom: 1px solid var(--border); user-select: none; }
.close-control { width: 23px; height: 30px; border: 0; background: transparent; display: grid; place-items: center; cursor: pointer; color: #7d0000; }
.close-control::before { content: ''; position: absolute; width: 14px; height: 14px; border-radius: 50%; background: #ff5f57; border: .5px solid #e0443e; }
.close-control :deep(svg) { position: relative; z-index: 1; opacity: 0; transition: opacity 120ms ease; }
.close-control:hover :deep(svg), .close-control:focus-visible :deep(svg) { opacity: 1; }
.settings-brand { display: flex; align-items: center; gap: 8px; }
.settings-brand b { font: 600 13px/1 var(--font-display); }
.settings-brand > span:last-child { color: var(--faint); font-size: 10px; }
.brand-icon { width: 22px; height: 22px; border-radius: 7px; display: grid; place-items: center; color: white; background: linear-gradient(135deg, var(--accent), oklch(64% .13 260)); }
.title-spacer { flex: 1; }
.settings-layout { flex: 1; min-height: 0; display: grid; grid-template-columns: 208px minmax(0, 1fr); }
.settings-nav { display: flex; flex-direction: column; gap: 4px; padding: 14px 12px; border-right: 1px solid var(--border); background: color-mix(in oklch, var(--glass-strong) 72%, transparent); }
.nav-title { padding: 4px 10px 10px; color: var(--faint); font: 500 10px/1 var(--mono); letter-spacing: .12em; }
.nav-item { position: relative; display: flex; align-items: center; gap: 11px; width: 100%; min-height: 52px; padding: 10px 11px; border: 1px solid transparent; border-radius: 10px; background: transparent; color: var(--muted); cursor: pointer; font: 550 12px/1 var(--font); text-align: left; transition: background 160ms ease, color 160ms ease, border-color 160ms ease; }
.nav-item:hover { background: var(--glass-hi); color: var(--fg); }
.nav-item.active { color: var(--fg); border-color: color-mix(in oklch, var(--accent) 30%, transparent); background: var(--accent-dim); }
.nav-item.active::before { content: ''; position: absolute; left: 0; top: 12px; bottom: 12px; width: 3px; border-radius: 0 3px 3px 0; background: var(--accent); }
.nav-icon { width: 30px; height: 30px; flex: none; display: grid; place-items: center; border: 1px solid var(--border); border-radius: 8px; background: color-mix(in oklch, var(--surface-muted) 60%, transparent); color: var(--muted); transition: inherit; }
.nav-item.active .nav-icon { color: var(--accent); border-color: color-mix(in oklch, var(--accent) 35%, transparent); background: color-mix(in oklch, var(--accent) 12%, transparent); }
.nav-label { color: var(--fg); font: 550 13px/1.2 var(--font); }
.nav-item:focus-visible, .close-control:focus-visible { outline: 2px solid var(--focus); outline-offset: 2px; }
.settings-content { min-width: 0; overflow-y: auto; padding: 20px 22px; }
.settings-content > [role='tabpanel'][hidden] { display: none; }
.settings-content::-webkit-scrollbar { width: 8px; }
.settings-content::-webkit-scrollbar-thumb { border: 2px solid transparent; border-radius: 8px; background: var(--border-hi); background-clip: padding-box; }
@media (max-width: 640px) {
  .settings-layout { grid-template-columns: 160px minmax(0, 1fr); }
  .settings-nav { padding-inline: 7px; }
  .nav-item { gap: 7px; padding-inline: 8px; }
  .settings-content { padding: 18px 14px 22px; }
}
@media (prefers-reduced-motion: reduce) { .nav-item, .close-control :deep(svg) { transition: none; } }
</style>
