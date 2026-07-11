<script setup lang="ts">
import { ref, watch } from 'vue'
import { useLyricDrag } from '@/composables/useLyricDrag'
import { useLyricMode } from '@/composables/useLyricMode'
import { useTranslator } from '@/composables/useTranslator'
import LyricControls from './LyricControls.vue'
import LyricBox from './LyricBox.vue'

const barEl = ref<HTMLElement | null>(null)

const peekActive = ref(false)
let peekTimer: ReturnType<typeof setTimeout> | undefined
function peek(ms = 2200) {
  peekActive.value = true
  clearTimeout(peekTimer)
  peekTimer = setTimeout(() => { peekActive.value = false }, ms)
}
function onEnter() { clearTimeout(peekTimer) }
function onLeave() {
  clearTimeout(peekTimer)
  peekActive.value = false
}

const { placed, dragging, pos, size, onDragBarMousedown, onResizeMousedown } = useLyricDrag(barEl, () => peek())

const { peekSignal } = useLyricMode()
watch(peekSignal, () => peek())

const { flip, swapText } = useTranslator()
const swapping = ref(false)
function onFlip() {
  swapping.value = true
  setTimeout(() => { swapText(); swapping.value = false }, 180)
  flip()
}
</script>

<template>
  <div
    ref="barEl"
    class="lyric-bar"
    :class="{ placed, dragging, peek: peekActive }"
    :style="{
      ...(placed ? { left: pos.x + 'px', top: pos.y + 'px' } : {}),
      ...(size.width ? { width: size.width + 'px' } : {}),
    }"
    @mouseenter="onEnter"
    @mouseleave="onLeave"
  >
    <LyricControls @flip="onFlip" />
    <LyricBox
      :swap="swapping"
      @dragbar-mousedown="onDragBarMousedown"
      @resize-mousedown="onResizeMousedown"
    />
  </div>
</template>

<style scoped>
.lyric-bar {
  position: fixed;
  left: 50%; top: 40px;
  transform: translateX(-50%);
  width: min(760px, calc(100vw - 48px));
  z-index: 40;
  display: flex;
  flex-direction: column;
  align-items: stretch;
}
.lyric-bar.placed { transform: none; }
.lyric-bar.dragging { user-select: none; }
.lyric-bar.dragging :deep(.lyric-ctrl) { opacity: 0 !important; }

.lyric-bar:hover :deep(.lyric-ctrl),
.lyric-bar.peek :deep(.lyric-ctrl) {
  opacity: 1; transform: translateY(0); pointer-events: auto;
}
</style>
