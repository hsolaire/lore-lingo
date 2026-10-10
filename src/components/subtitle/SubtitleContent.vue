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
  <div class="subtitle-content" :class="{ swap }">
    <div class="drag-bar" @mousedown="emit('dragbar-mousedown', $event)" />
    <p v-if="srcText && mode !== 'translated'" class="subtitle-source">{{ srcText }}</p>
    <p v-if="mode !== 'original'" class="subtitle-target" :class="{ empty: !tgtText }">{{ tgtText || '译文会显示在这里' }}</p>
    <p v-if="!srcText && !tgtText" class="subtitle-hint">移入显示工具栏，选择捕获来源后开始翻译</p>
    <div class="resize-handle left"  @mousedown="emit('resize-mousedown', $event, 'left')" />
    <div class="resize-handle right" @mousedown="emit('resize-mousedown', $event, 'right')" />
  </div>
</template>

<style scoped>
.subtitle-content {
  position: relative;
  padding: 24px 26px 26px;
  text-align: center;
  overflow: hidden;
}
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
.subtitle-target.empty { font-size: 20px; color: var(--muted); font-weight: 500; }
.subtitle-hint { font-size: 11px; color: var(--faint); margin-top: 8px; }
.subtitle-content.swap .subtitle-source,
.subtitle-content.swap .subtitle-target { opacity: 0; }

.resize-handle {
  position: absolute;
  bottom: 0;
  width: 40px;
  height: 18px;
  cursor: ew-resize;
  -webkit-app-region: no-drag;
  display: flex;
  align-items: flex-end;
  justify-content: center;
  padding-bottom: 4px;
}
.resize-handle::before {
  content: "";
  width: 24px;
  height: 4px;
  border-radius: 2px;
  background: var(--border-hi);
  transition: background .15s;
}
.resize-handle:hover::before { background: var(--accent); }
.resize-handle.left  { left: 14px; }
.resize-handle.right { right: 14px; }
</style>
