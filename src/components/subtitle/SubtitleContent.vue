<script setup lang="ts">
import { useTranslator } from '@/composables/useTranslator'

defineProps<{ swap?: boolean }>()

const emit = defineEmits<{
  (e: 'dragbar-mousedown', ev: MouseEvent): void
  (e: 'resize-mousedown', ev: MouseEvent, side: 'left' | 'right'): void
}>()

const { srcText, tgtText } = useTranslator()
</script>

<template>
  <div class="subtitle-content" :class="{ swap }">
    <div class="drag-bar" @mousedown="emit('dragbar-mousedown', $event)" />
    <p class="subtitle-source">{{ srcText }}</p>
    <p class="subtitle-target">{{ tgtText }}</p>
    <div class="resize-handle left"  @mousedown="emit('resize-mousedown', $event, 'left')" />
    <div class="resize-handle right" @mousedown="emit('resize-mousedown', $event, 'right')" />
  </div>
</template>

<style scoped>
.subtitle-content {
  position: relative;
  background: oklch(20% 0.02 260 / 0.62);
  backdrop-filter: blur(44px) saturate(1.8);
  -webkit-backdrop-filter: blur(44px) saturate(1.8);
  border: 1px solid var(--border-hi);
  border-radius: 18px;
  padding: 68px 26px 24px;
  text-align: center;
  overflow: hidden;
}
.subtitle-content::before {
  content: ""; position: absolute; inset: 0 0 auto 0; height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-hi), transparent);
  border-radius: 18px 18px 0 0;
}

.drag-bar {
  position: absolute;
  inset: 0 0 auto 0;
  height: 28px;
  border-radius: 18px 18px 0 0;
  cursor: grab;
}
.drag-bar:active { cursor: grabbing; }

.subtitle-source {
  font: 500 16px/1.45 var(--font); color: var(--muted); letter-spacing: 0.01em;
  margin-bottom: 7px; transition: opacity .18s;
  -webkit-app-region: no-drag;
}
.subtitle-target {
  font: 600 30px/1.35 var(--font-display); color: var(--fg); letter-spacing: -0.015em;
  text-wrap: balance; transition: opacity .18s;
  -webkit-app-region: no-drag;
}
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
