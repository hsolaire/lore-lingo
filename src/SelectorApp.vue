<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'

import { captureTranslationClient } from '@/services/captureTranslationClient'
import type { LogicalRect, SelectorPayload } from '@/types/capturedLayout'
import { createLogicalRect } from '@/selectorGeometry'

type Point = { x: number; y: number }

const payload = ref<SelectorPayload | null>(null)
const start = ref<Point | null>(null)
const current = ref<Point | null>(null)
const submitting = ref(false)
const error = ref('')

const imageUrl = computed(() => payload.value ? `data:image/png;base64,${payload.value.imagePngB64}` : '')
const selection = computed<LogicalRect | null>(() => {
  return createLogicalRect(start.value, current.value)
})

const selectionStyle = computed(() => {
  if (!selection.value) return {}
  return {
    left: `${selection.value.x}px`,
    top: `${selection.value.y}px`,
    width: `${selection.value.width}px`,
    height: `${selection.value.height}px`,
  }
})

function pointFromEvent(event: PointerEvent): Point {
  const width = payload.value?.logicalWidth ?? window.innerWidth
  const height = payload.value?.logicalHeight ?? window.innerHeight
  return {
    x: Math.max(0, Math.min(event.clientX, width)),
    y: Math.max(0, Math.min(event.clientY, height)),
  }
}

function beginSelection(event: PointerEvent) {
  if (submitting.value || event.button !== 0) return
  const point = pointFromEvent(event)
  start.value = point
  current.value = point
  ;(event.currentTarget as HTMLElement).setPointerCapture(event.pointerId)
}

function updateSelection(event: PointerEvent) {
  if (!start.value || submitting.value) return
  current.value = pointFromEvent(event)
}

async function finishSelection(event: PointerEvent) {
  if (!start.value || !payload.value || submitting.value) return
  current.value = pointFromEvent(event)
  const rect = selection.value
  start.value = null
  current.value = null
  if (!rect || rect.width < 4 || rect.height < 4) return

  submitting.value = true
  try {
    await captureTranslationClient.complete(payload.value.sessionId, payload.value.monitorId, rect)
  } catch (reason: unknown) {
    error.value = typeof reason === 'object' && reason && 'message' in reason
      ? String(reason.message)
      : '无法提交所选区域，请按 Escape 取消后重试。'
    submitting.value = false
  }
}

async function cancel() {
  if (!payload.value || submitting.value) return
  submitting.value = true
  try {
    await captureTranslationClient.cancel(payload.value.sessionId)
  } catch {
    submitting.value = false
    error.value = '无法取消当前选择，请重试。'
  }
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') void cancel()
}

onMounted(async () => {
  window.addEventListener('keydown', onKeydown)
  try {
    payload.value = await captureTranslationClient.selectorPayload()
  } catch {
    error.value = '无法读取冻结画面，请按 Escape 返回。'
  }
})

onUnmounted(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <main
    class="selector"
    :aria-busy="submitting"
    @pointerdown="beginSelection"
    @pointermove="updateSelection"
    @pointerup="finishSelection"
  >
    <img v-if="payload" :src="imageUrl" alt="冻结的显示器画面" draggable="false" @error="error = '无法显示冻结截图，请取消后重试。'" />
    <div class="shade" />
    <div v-if="selection" class="selection" :style="selectionStyle">
      <span>{{ Math.round(selection.width) }} × {{ Math.round(selection.height) }}</span>
    </div>
    <div class="instructions">
      <b>{{ submitting ? '正在处理所选区域…' : '拖拽框选要翻译的区域' }}</b>
      <span>单次选择仅限当前显示器 · Esc 取消</span>
    </div>
    <button type="button" @pointerdown.stop @click.stop="cancel">取消</button>
    <p v-if="error" class="error" role="alert">{{ error }}</p>
  </main>
</template>

<style scoped>
.selector { position: fixed; inset: 0; overflow: hidden; cursor: crosshair; user-select: none; background: #080a0d; color: #fff; }
.selector > img, .shade { position: absolute; inset: 0; width: 100%; height: 100%; }
.selector > img { object-fit: fill; pointer-events: none; }.shade { background: rgb(4 7 12 / 24%); pointer-events: none; }
.selection { position: absolute; z-index: 2; border: 2px solid #62c7f7; background: rgb(70 181 235 / 12%); box-shadow: 0 0 0 9999px rgb(3 6 10 / 24%), 0 0 0 1px rgb(255 255 255 / 70%) inset; }
.selection span { position: absolute; left: 8px; bottom: 8px; padding: 5px 7px; border-radius: 6px; background: rgb(8 13 19 / 86%); font: 600 12px/1 ui-monospace, monospace; white-space: nowrap; }
.instructions { position: absolute; z-index: 3; top: 24px; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; align-items: center; gap: 5px; padding: 10px 16px; border: 1px solid rgb(255 255 255 / 20%); border-radius: 11px; background: rgb(10 14 20 / 82%); box-shadow: 0 12px 36px rgb(0 0 0 / 45%); backdrop-filter: blur(14px); pointer-events: none; }
.instructions b { font: 650 14px/1.2 -apple-system, sans-serif; }.instructions span { color: #c2cbd4; font: 12px/1.2 -apple-system, sans-serif; }
button { position: absolute; z-index: 4; top: 24px; right: 24px; border: 1px solid rgb(255 255 255 / 24%); border-radius: 9px; padding: 9px 14px; background: rgb(10 14 20 / 82%); color: #fff; font: 600 13px/1 -apple-system, sans-serif; cursor: pointer; }
button:hover, button:focus-visible { border-color: #62c7f7; outline: 2px solid #62c7f7; outline-offset: 2px; }
.error { position: absolute; z-index: 4; left: 50%; bottom: 24px; transform: translateX(-50%); max-width: min(560px, calc(100vw - 48px)); padding: 10px 14px; border: 1px solid rgb(255 120 120 / 45%); border-radius: 9px; background: rgb(65 16 20 / 90%); font: 13px/1.4 -apple-system, sans-serif; }
@media (prefers-reduced-motion: reduce) { * { transition: none !important; } }
</style>
