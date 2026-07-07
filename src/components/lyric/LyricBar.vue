<script setup lang="ts">
import { ref, watch } from 'vue'
import { useLyricDrag } from '@/composables/useLyricDrag'
import { useLyricMode } from '@/composables/useLyricMode'
import { useTranslator } from '@/composables/useTranslator'
import LyricControls from './LyricControls.vue'
import LyricBox from './LyricBox.vue'

const barEl = ref<HTMLElement | null>(null)

// 控制排 peek：移开后短暂驻留再隐藏
const peekActive = ref(false)
let peekTimer: ReturnType<typeof setTimeout> | undefined
function peek(ms = 2200) {
  peekActive.value = true
  clearTimeout(peekTimer)
  peekTimer = setTimeout(() => { peekActive.value = false }, ms)
}
function onEnter() {
  clearTimeout(peekTimer)
}
function onLeave() {
  clearTimeout(peekTimer)
  peekActive.value = false
}

const { placed, dragging, pos, onBoxMousedown } = useLyricDrag(barEl, () => peek())

// 进入歌词条时触发一次 peek
const { peekSignal } = useLyricMode()
watch(peekSignal, () => peek())

// 翻转动画：淡出 → 交换文本与语言方向 → 淡入
const { flip, swapText } = useTranslator()
const swapping = ref(false)
function onFlip() {
  swapping.value = true
  setTimeout(() => {
    swapText()
    swapping.value = false
  }, 180)
  flip()
}
</script>

<template>
  <div
    ref="barEl"
    class="lyric-bar"
    :class="{ placed, dragging, peek: peekActive }"
    :style="placed ? { left: pos.x + 'px', top: pos.y + 'px' } : undefined"
    @mouseenter="onEnter"
    @mouseleave="onLeave"
  >
    <LyricControls @flip="onFlip" />
    <div @mousedown="onBoxMousedown">
      <LyricBox :swap="swapping" />
    </div>
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
  -webkit-app-region: drag;
  cursor: grab;
}
/* 拖拽定位：一旦用户拖过，用 left/top 精确定位并取消居中 translate */
.lyric-bar.placed { transform: none; }
.lyric-bar.dragging { cursor: grabbing; user-select: none; }
.lyric-bar.dragging :deep(.lyric-ctrl) { opacity: 0 !important; }

/* hover 歌词条任意处 → 显示控制排；.peek 是移开后短暂驻留 */
.lyric-bar:hover :deep(.lyric-ctrl),
.lyric-bar.peek :deep(.lyric-ctrl) {
  opacity: 1; transform: translateY(0); pointer-events: auto;
}
</style>
