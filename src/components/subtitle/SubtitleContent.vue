<script setup lang="ts">
import { useTranslator } from '@/composables/useTranslator'

withDefaults(defineProps<{ swap?: boolean; mode?: 'original' | 'translated' | 'bilingual' }>(), { mode: 'bilingual' })

const emit = defineEmits<{
  (e: 'dragbar-mousedown', ev: MouseEvent): void
  (e: 'resize-mousedown', ev: MouseEvent, side: 'left' | 'right'): void
}>()

const { srcText, tgtText } = useTranslator()
</script>

<template>
  <div class="subtitle-content" :class="{ swap, 'is-empty': !srcText && !tgtText }">
    <div class="drag-bar" @mousedown="emit('dragbar-mousedown', $event)" />
    <p v-if="srcText && mode !== 'translated'" class="subtitle-source">{{ srcText }}</p>
    <p v-if="mode !== 'original'" class="subtitle-target" :class="{ empty: !tgtText }">{{ tgtText || '译文会显示在这里' }}</p>
    <p v-if="!srcText && !tgtText" class="subtitle-hint">移入显示工具栏，选择捕获来源后开始翻译</p>
    <div class="resize-handle left" title="拖动调整窗口宽度" aria-hidden="true" @mousedown="emit('resize-mousedown', $event, 'left')" />
    <div class="resize-handle right" title="拖动调整窗口宽度" aria-hidden="true" @mousedown="emit('resize-mousedown', $event, 'right')" />
  </div>
</template>

<style scoped>
.subtitle-content {
  position: relative;
  padding: 22px 30px 26px;
  text-align: center;
  overflow: hidden;
}
.is-empty { min-height: 104px; display: flex; flex-direction: column; justify-content: center; gap: 7px; padding-block: 22px; }
.drag-bar {
  position: absolute;
  inset: 0 0 auto 0;
  height: 18px;
  border-radius: 18px 18px 0 0;
  cursor: grab;
}
.drag-bar:active { cursor: grabbing; }

.subtitle-source {
  font: 500 16px/1.45 var(--font); color: var(--muted); letter-spacing: 0.01em;
  margin-bottom: 7px; transition: opacity .18s;
  -webkit-app-region: no-drag;
  white-space: pre-line;
  max-height: 140px;
  overflow-y: auto;
}
.subtitle-target {
  font: 600 30px/1.35 var(--font-display); color: var(--fg); letter-spacing: -0.015em;
  text-wrap: balance; transition: opacity .18s;
  -webkit-app-region: no-drag;
  white-space: pre-line;
  max-height: 260px;
  overflow-y: auto;
}
.subtitle-target.empty { font-size: 18px; color: var(--muted); font-weight: 500; letter-spacing: 0; }
.subtitle-hint { font-size: 12px; line-height: 1.5; color: var(--faint); }
.subtitle-content.swap .subtitle-source,
.subtitle-content.swap .subtitle-target { opacity: 0; }

.resize-handle {
  position: absolute;
  bottom: 0;
  width: 28px;
  height: 28px;
  cursor: ew-resize;
  -webkit-app-region: no-drag;
  display: flex;
  align-items: center;
  justify-content: center;
}
.resize-handle::before {
  content: "";
  width: 8px;
  height: 8px;
  border-bottom: 1.5px solid var(--muted);
  opacity: 0;
  transition: opacity .16s, border-color .16s;
}
.subtitle-content:hover .resize-handle::before { opacity: .3; }
.resize-handle:hover::before { opacity: 1 !important; border-color: var(--accent); }
.resize-handle.left { left: 0; }
.resize-handle.left::before { border-left: 1.5px solid var(--muted); border-bottom-left-radius: 3px; }
.resize-handle.right { right: 0; }
.resize-handle.right::before { border-right: 1.5px solid var(--muted); border-bottom-right-radius: 3px; }
</style>
