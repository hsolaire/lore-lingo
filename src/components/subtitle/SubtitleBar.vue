<script setup lang="ts">
import { ref, watch, onUnmounted } from 'vue'
import { useSubtitleWindow } from '@/composables/useSubtitleWindow'
import { useTranslator } from '@/composables/useTranslator'
import SubtitleControls from './SubtitleControls.vue'
import SubtitleContent from './SubtitleContent.vue'

const props = defineProps<{ externalPeek?: number }>()

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

const {
  pinned,
  onDragBarMousedown,
  onResizeMousedown,
  togglePinned,
} = useSubtitleWindow(barEl)

watch(() => props.externalPeek, () => peek())

const { flip, swapText } = useTranslator()
const swapping = ref(false)
let swapTimer: ReturnType<typeof setTimeout> | undefined
function onFlip() {
  swapping.value = true
  clearTimeout(swapTimer)
  swapTimer = setTimeout(() => { swapText(); swapping.value = false }, 180)
  flip()
}

onUnmounted(() => {
  clearTimeout(peekTimer)
  clearTimeout(swapTimer)
})
</script>

<template>
  <div
    ref="barEl"
    class="subtitle-bar"
    :class="{ peek: peekActive }"
    @mouseenter="onEnter"
    @mouseleave="onLeave"
  >
    <SubtitleControls :pinned="pinned" @flip="onFlip" @toggle-pinned="togglePinned" />
    <SubtitleContent
      :swap="swapping"
      @dragbar-mousedown="onDragBarMousedown"
      @resize-mousedown="onResizeMousedown"
    />
  </div>
</template>

<style scoped>
.subtitle-bar {
  position: relative;
  width: 100%;
}

.subtitle-bar:hover :deep(.subtitle-controls),
.subtitle-bar.peek :deep(.subtitle-controls) {
  opacity: 1; transform: translate(-50%, 0); pointer-events: auto;
}
</style>
