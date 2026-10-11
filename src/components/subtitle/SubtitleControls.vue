<script setup lang="ts">
import { ArrowUpToLine, Check, ChevronDown, Crop, Languages, LockKeyhole, Monitor, Pause, Settings, UnlockKeyhole, Minus, X } from 'lucide-vue-next'
import { NButton, NIcon } from 'naive-ui'
import type { PlatformKind } from '@/types/desktop'
import { currentWindowClient } from '@/services/windowClient'

withDefaults(defineProps<{
  platform?: PlatformKind
  toolbarPinned?: boolean
  alwaysOnTop?: boolean
  sourceLabel?: string
  languageLabel?: string
  busy?: boolean
  captured?: boolean
  optionsOpen?: boolean
  windowsOpen?: boolean
}>(), {
  platform: 'unsupported', toolbarPinned: false, alwaysOnTop: true,
  sourceLabel: '选择捕获窗口', languageLabel: 'auto → zh-CN',
  busy: false, captured: false, optionsOpen: false, windowsOpen: false,
})

const emit = defineEmits<{
  options: []; source: []; translate: []; settings: []
  preview: [show: boolean]
  'toggle-toolbar-pinned': []; 'toggle-always-on-top': []
}>()

function drag(event: MouseEvent) {
  if (event.button !== 0 || (event.target instanceof Element && event.target.closest('button'))) return
  void currentWindowClient.startDragging().catch(() => {})
}
</script>

<template>
  <header class="subtitle-controls" role="toolbar" aria-label="悬浮字幕工具栏" @mousedown="drag">
    <span v-if="platform === 'macos'" class="native-space" aria-hidden="true" />
    <div class="action-group" role="group" aria-label="翻译操作">
      <NButton size="small" quaternary aria-label="翻译选项" aria-haspopup="dialog" :aria-expanded="optionsOpen" :title="`翻译选项：${languageLabel}`" @click="emit('options')">
        <template #icon><NIcon :component="Languages" /></template>
        <span class="label">翻译</span><ChevronDown :size="12" />
      </NButton>
      <NButton size="small" quaternary :disabled="busy" aria-label="捕获窗口" aria-haspopup="dialog" :aria-expanded="windowsOpen" :title="sourceLabel" @click="emit('source')">
        <template #icon><NIcon :component="Monitor" /></template>
        <span class="label">窗口</span><ChevronDown :size="12" />
      </NButton>
      <NButton size="small" :type="captured ? 'success' : 'primary'" :aria-label="busy ? '取消捕获' : captured ? '编辑捕获区域' : '捕获区域'" :title="busy ? '取消当前任务' : captured ? '已捕获，悬停实时预览，点击编辑坐标' : '框选要翻译的区域'" @click="emit('translate')" @mouseenter="emit('preview', true)" @mouseleave="emit('preview', false)" @focus="emit('preview', true)" @blur="emit('preview', false)">
        <template #icon><NIcon :component="busy ? Pause : captured ? Check : Crop" /></template>
        <span class="label">{{ busy ? '取消' : captured ? '已捕获' : '区域' }}</span>
      </NButton>
      <NButton quaternary circle size="small" aria-label="设置" title="设置" @click="emit('settings')">
        <template #icon><NIcon :component="Settings" /></template>
      </NButton>
    </div>
    <div class="window-group" role="group" aria-label="窗口操作">
      <NButton quaternary circle size="small" :class="{ active: alwaysOnTop }" aria-label="窗口置顶" :aria-pressed="alwaysOnTop" title="窗口置顶" @click="emit('toggle-always-on-top')">
        <template #icon><NIcon :component="ArrowUpToLine" /></template>
      </NButton>
      <NButton quaternary circle size="small" :class="{ active: toolbarPinned }" aria-label="固定工具栏" :aria-pressed="toolbarPinned" :title="toolbarPinned ? '取消固定工具栏' : '固定工具栏'" @click="emit('toggle-toolbar-pinned')">
        <template #icon><NIcon :component="toolbarPinned ? LockKeyhole : UnlockKeyhole" /></template>
      </NButton>
      <template v-if="platform !== 'macos'">
        <NButton quaternary circle size="small" aria-label="最小化窗口" title="最小化窗口" @click="currentWindowClient.minimize()"><template #icon><NIcon :component="Minus" /></template></NButton>
        <NButton quaternary circle size="small" aria-label="关闭窗口" title="关闭窗口" @click="currentWindowClient.close()"><template #icon><NIcon :component="X" /></template></NButton>
      </template>
    </div>
  </header>
</template>

<style scoped>
.subtitle-controls { display: flex; align-items: center; gap: 16px; padding: 7px 12px; min-height: 46px; border-bottom: 1px solid var(--border); background: var(--surface-muted); color: var(--muted); user-select: none; }
.native-space { width: 70px; height: 28px; flex: none; }
.action-group, .window-group { display: flex; align-items: center; gap: 6px; flex: none; }
.window-group { margin-left: auto; }
:deep(.n-button) { height: 30px; flex: none; border-radius: 7px; }
:deep(.n-button--circle) { width: 30px; }
:deep(.n-button__content) { gap: 5px; }
:deep(.n-icon) { font-size: 18px; }
.active { color: var(--accent); background: var(--surface-muted); }
:deep(button:focus-visible) { outline: 2px solid var(--focus); outline-offset: 2px; }
@media (max-width: 580px) { .subtitle-controls { gap: 10px; padding-inline: 8px; }.action-group, .window-group { gap: 4px; } }
@media (max-width: 540px) { .label { display: none; } }
</style>
