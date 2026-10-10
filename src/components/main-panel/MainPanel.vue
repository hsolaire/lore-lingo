<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue'
import { NButton, NEmpty, NSelect, NSpin } from 'naive-ui'
import SubtitleControls from '@/components/subtitle/SubtitleControls.vue'
import SubtitleContent from '@/components/subtitle/SubtitleContent.vue'
import LayoutResultWorkspace from './LayoutResultWorkspace.vue'
import RegionCapturePanel from './RegionCapturePanel.vue'
import { useCaptureTranslation } from '@/composables/useCaptureTranslation'
import { useCaptureSource } from '@/composables/useCaptureSource'
import { useSubtitleWindow } from '@/composables/useSubtitleWindow'
import { useOverlayToolbar } from '@/composables/useOverlayToolbar'
import { useTranslator } from '@/composables/useTranslator'
import { useWindowNavigation } from '@/composables/useWindowNavigation'
import { windowClient } from '@/services/windowClient'
import { useToast } from '@/composables/useToast'
import type { PlatformKind, CaptureSourceSummary } from '@/types/desktop'

const props = withDefaults(defineProps<{ platform?: PlatformKind }>(), { platform: 'unsupported' })
const root = ref<HTMLElement | null>(null)
const menu = ref<'options' | 'windows' | 'preview' | null>(null)
const showResult = ref(false)
const displayMode = ref<'original' | 'translated' | 'bilingual'>('bilingual')
const capture = useCaptureTranslation()
const { sourceMode, targetName, windowList, selectedWindow, listLoading, fetchWindows, selectWindow } = useCaptureSource()
const { srcText, tgtText, langA, langB, flip } = useTranslator()
const { openSettings } = useWindowNavigation('main')
const { toast } = useToast()
const keepOpen = computed(() => !!menu.value || showResult.value || capture.busy.value)
const { visible, toolbarPinned, reveal, leave, toggleToolbarPinned, onFocusIn, onFocusOut } = useOverlayToolbar(keepOpen)
const { pinned, togglePinned, onDragBarMousedown, onResizeMousedown } = useSubtitleWindow(root)
const document = computed(() => capture.state.value.document)
let previewTimer: ReturnType<typeof setTimeout> | undefined
let previewSticky = false
let suppressPreview = false
const canShowResult = computed(() => !!document.value)
const languages = [
  { label: '自动检测', value: 'auto' }, { label: '简体中文', value: 'zh-CN' },
  { label: '繁体中文', value: 'zh-TW' }, { label: 'English', value: 'en' },
  { label: '日本語', value: 'ja' }, { label: '한국어', value: 'ko' },
]
const languageMenuTheme = { peers: { InternalSelectMenu: { height: '128px' } } }
const targetLanguages = languages.filter(language => language.value !== 'auto')
const displayModes = [{ label: '原文', value: 'original' }, { label: '译文', value: 'translated' }, { label: '双语', value: 'bilingual' }]
const statusMessage = computed(() => {
  const state = capture.state.value
  if (state.status === 'error') return `${document.value ? '已捕获。' : ''}${state.error.message}`
  if (state.status === 'empty') return '已捕获，未识别到文字；悬停区域按钮可预览。'
  if (state.status === 'processing') return `${document.value ? '已捕获，' : ''}正在识别并翻译…`
  return ''
})

function toggleMenu(next: 'options' | 'windows') {
  clearTimeout(previewTimer)
  previewSticky = false
  menu.value = menu.value === next ? null : next
  if (menu.value === 'windows') void fetchWindows()
}

function translate() {
  if (!capture.busy.value && sourceMode.value === 'region' && document.value) {
    previewSticky = !previewSticky
    menu.value = previewSticky ? 'preview' : null
    return
  }
  reselectRegion()
}

function reselectRegion() {
  clearTimeout(previewTimer)
  previewSticky = false
  menu.value = null
  showResult.value = false
  if (capture.busy.value) void capture.cancel()
  else { sourceMode.value = 'region'; void capture.start() }
}

async function captureWindow(window: CaptureSourceSummary) {
  selectWindow(window)
  sourceMode.value = 'app'
  menu.value = null
  showResult.value = false
  await capture.startWindow(window)
}

function translateRegion(revision: number) {
  menu.value = null
  sourceMode.value = 'region'
  void capture.translateRegion(revision)
}

function retryCapture() {
  if (sourceMode.value === 'app' && selectedWindow.value) void captureWindow(selectedWindow.value)
  else reselectRegion()
}

function flipLanguage() { flip(); if (langB.value === 'auto') langB.value = 'en' }

async function copyText(text: string, label: string) {
  try { await navigator.clipboard.writeText(text); toast(`已复制${label}`) }
  catch { toast('复制失败，请重试。') }
}

function closePanels() {
  clearTimeout(previewTimer)
  previewSticky = false
  const previous = menu.value
  menu.value = null
  showResult.value = false
  if (previous === 'preview') {
    suppressPreview = true
    root.value?.querySelector<HTMLButtonElement>('button[aria-label="编辑捕获区域"]')?.focus()
    suppressPreview = false
  }
  if (previous === 'options' || previous === 'windows') {
    root.value?.querySelector<HTMLButtonElement>(`button[aria-label="${previous === 'options' ? '翻译选项' : '捕获窗口'}"]`)?.focus()
  }
}

function keepPreviewOpen() { clearTimeout(previewTimer) }

function showPreview(show: boolean) {
  if (show && suppressPreview) return
  clearTimeout(previewTimer)
  if (show && sourceMode.value === 'region' && document.value && !menu.value) menu.value = 'preview'
  else if (!show && menu.value === 'preview' && !previewSticky && !root.value?.querySelector('.preview-menu')?.contains(globalThis.document.activeElement)) previewTimer = setTimeout(() => { menu.value = null }, 250)
}

onUnmounted(() => clearTimeout(previewTimer))

// 原生交通灯和工具栏共用显示状态，弹出菜单与预览期间保持可操作。
watch([visible, () => props.platform], ([shown, platform]) => {
  if (platform === 'macos') void windowClient.setMainControlsVisible(shown).catch(() => toast('窗口按钮显示状态更新失败，请重试。'))
}, { immediate: true })
</script>

<template>
  <main ref="root" class="overlay-host" :class="{ 'toolbar-visible': visible }" tabindex="0" aria-label="LoreLingo 悬浮字幕" @pointerenter="reveal" @pointerleave="leave" @focusin="onFocusIn" @focusout="onFocusOut" @keydown.esc.stop="closePanels">
    <div class="unified-overlay subtitle-bar">
      <SubtitleControls v-show="visible" :platform="platform" :toolbar-pinned="toolbarPinned" :always-on-top="pinned" :source-label="targetName" :language-label="`${langA} → ${langB}`" :busy="capture.busy.value" :captured="sourceMode === 'region' && !!document" :options-open="menu === 'options'" :windows-open="menu === 'windows'" @options="toggleMenu('options')" @source="toggleMenu('windows')" @translate="translate" @settings="menu = null; openSettings()" @preview="showPreview" @toggle-toolbar-pinned="toggleToolbarPinned" @toggle-always-on-top="togglePinned" />
      <section class="subtitle-surface" aria-label="字幕区域">
        <p v-if="statusMessage" class="capture-status" :class="{ error: capture.state.value.status === 'error' }" aria-live="polite">{{ statusMessage }}</p>
        <LayoutResultWorkspace v-if="showResult" @configure="openSettings" @retry="retryCapture" @close="showResult = false" />
        <SubtitleContent v-else :mode="displayMode" @dragbar-mousedown="onDragBarMousedown" @resize-mousedown="onResizeMousedown" />
      </section>
    </div>
    <section v-if="menu === 'options'" class="toolbar-menu options-menu" role="dialog" aria-label="翻译选项" @keydown.esc.stop="closePanels">
      <label>源语言<NSelect v-model:value="langA" aria-label="源语言" size="small" :theme-overrides="languageMenuTheme" :options="languages" :to="false" /></label>
      <label>目标语言<NSelect v-model:value="langB" aria-label="目标语言" size="small" :theme-overrides="languageMenuTheme" :options="targetLanguages" :to="false" /></label>
      <NButton size="small" @click="flipLanguage">交换翻译方向</NButton>
      <label>字幕显示<NSelect v-model:value="displayMode" aria-label="字幕显示" size="small" :options="displayModes" :to="false" /></label>
      <div class="menu-actions">
        <NButton size="small" :disabled="!srcText" @click="copyText(srcText, '原文')">复制原文</NButton>
        <NButton size="small" :disabled="!tgtText" @click="copyText(tgtText, '译文')">复制译文</NButton>
        <NButton size="small" :disabled="!canShowResult" @click="showResult = !showResult; menu = null">查看版面</NButton>
      </div>
    </section>
    <section v-else-if="menu === 'windows'" class="toolbar-menu windows-menu" role="dialog" aria-label="选择捕获窗口" @keydown.esc.stop="closePanels">
      <header><b>捕获窗口</b><NButton size="tiny" :loading="listLoading" @click="fetchWindows">刷新</NButton></header>
      <NSpin v-if="listLoading" size="small" />
      <NEmpty v-else-if="!windowList.length" description="暂无可捕获窗口，请检查录屏权限后刷新" size="small" />
      <div v-else class="window-list">
        <NButton v-for="window in windowList" :key="window.sourceId" block quaternary :type="selectedWindow?.sourceId === window.sourceId ? 'primary' : 'default'" :title="window.title" @click="captureWindow(window)">
          <span>{{ window.title }}<small>{{ window.applicationName }}</small></span>
        </NButton>
      </div>
    </section>
    <div v-else-if="menu === 'preview' && document" class="toolbar-menu preview-menu" @mouseenter="keepPreviewOpen" @mouseleave="showPreview(false)" @focusin="keepPreviewOpen" @focusout="showPreview(false)">
      <RegionCapturePanel :busy="capture.busy.value" @reselect="reselectRegion" @translate="translateRegion" />
    </div>

  </main>
</template>

<style scoped>
.overlay-host { display: grid; width: 100%; min-width: 0; position: relative; outline: none; pointer-events: none; }
.unified-overlay { grid-area: 1 / 1; align-self: start; pointer-events: auto; min-height: 104px; display: flex; flex-direction: column; width: 100%; min-width: 0; max-height: 720px; background: var(--glass); border: 1px solid var(--border-hi); border-radius: 12px; backdrop-filter: blur(40px) saturate(1.6); -webkit-backdrop-filter: blur(40px) saturate(1.6); overflow: hidden; outline: none; position: relative; }
.subtitle-surface { min-height: 104px; flex: 1; min-width: 0; overflow: hidden; }
.overlay-host:focus-visible > .unified-overlay { border-color: var(--focus); }
.toolbar-menu { grid-area: 1 / 1; align-self: start; position: relative; pointer-events: auto; z-index: 10; margin: 52px 0 14px 90px; width: min(360px, calc(100% - 106px)); max-height: 305px; overflow-y: auto; padding: 14px; border: 1px solid var(--border-hi); border-radius: 10px; background: var(--glass-strong); box-shadow: var(--shadow); }
.options-menu { display: grid; gap: 10px; overflow: visible; }
.options-menu label { display: grid; grid-template-columns: 60px 1fr; align-items: center; gap: 10px; font-size: 12px; color: var(--muted); }
.menu-actions { display: flex; flex-wrap: wrap; gap: 6px; }
.windows-menu header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; font-size: 12px; }
.window-list { display: grid; gap: 4px; }
.window-list span { display: block; min-width: 0; width: 280px; text-align: left; overflow: hidden; text-overflow: ellipsis; }
.window-list small { display: block; font-size: 10px; color: var(--muted); }
.window-list :deep(.n-button) { height: 46px; }
.preview-menu { margin-left: min(240px, 20%); width: min(400px, calc(100% - min(240px, 20%) - 16px)); max-height: 480px; }
.capture-status { font-size: 12px; color: var(--muted); text-align: center; padding: 10px 20px 0; }
.capture-status.error { color: #ffb4b4; }
:deep(.result-workspace) { position: relative; inset: auto; height: 490px; }
@media (prefers-reduced-transparency: reduce) { .unified-overlay { background: var(--glass-strong); backdrop-filter: none; -webkit-backdrop-filter: none; } }
</style>
