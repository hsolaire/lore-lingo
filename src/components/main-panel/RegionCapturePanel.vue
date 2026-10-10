<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { NAlert, NButton, NInputNumber, NSpin } from 'naive-ui'
import { useRegionPreview } from '@/composables/useRegionPreview'
import type { RegionBounds } from '@/types/capturedLayout'

defineProps<{ busy: boolean }>()
const emit = defineEmits<{ reselect: []; translate: [revision: number] }>()
const preview = useRegionPreview()
const transitioning = ref(false)
const { region, frame, error, loading, saving } = preview
const draft = reactive<Record<keyof RegionBounds, number | null>>({ x1: null, x2: null, y1: null, y2: null })
const keys = ['x1', 'x2', 'y1', 'y2'] as const
watch(region, value => { if (value) Object.assign(draft, value.bounds) }, { immediate: true })
const valid = computed(() => {
  const { x1, x2, y1, y2 } = draft
  return !!region.value && keys.every(key => Number.isFinite(draft[key])) && x1 !== null && x2 !== null && y1 !== null && y2 !== null
    && x1 >= 0 && y1 >= 0 && x2 - x1 >= 4 && y2 - y1 >= 4 && x2 <= region.value.logicalWidth && y2 <= region.value.logicalHeight
})
const modified = computed(() => !!region.value && keys.some(key => draft[key] !== region.value?.bounds[key]))
const image = computed(() => frame.value ? `data:image/png;base64,${frame.value.imagePngB64}` : '')
const updatedTime = computed(() => frame.value ? new Date(frame.value.capturedAtMs).toLocaleTimeString() : '')
function apply() {
  if (valid.value) void preview.apply({ x1: draft.x1!, x2: draft.x2!, y1: draft.y1!, y2: draft.y2! })
}
async function finish(action: 'translate' | 'reselect') {
  if (transitioning.value) return
  transitioning.value = true
  await preview.stop()
  if (action === 'reselect') emit('reselect')
  else if (region.value) emit('translate', region.value.revision)
}
</script>

<template>
  <section class="region-panel" role="dialog" aria-label="实时区域预览与坐标">
    <header><b>{{ error ? '预览已暂停' : '实时区域预览' }}</b><span v-if="frame">{{ updatedTime }}</span></header>
    <div class="region-image">
      <img v-if="frame" :src="image" alt="当前捕获区域的实时画面" />
      <NSpin v-else-if="loading" size="small" />
      <span v-else>暂无区域画面</span>
    </div>
    <p v-if="frame">{{ frame.imageWidth }} × {{ frame.imageHeight }} 像素 · 每帧完成后 0.5 秒刷新</p>
    <p v-if="region">当前显示器左上角为原点，使用逻辑坐标。</p>
    <div class="coordinates">
      <label v-for="key in keys" :key="key">{{ key.toUpperCase() }}
        <NInputNumber v-model:value="draft[key]" size="small" :min="0" :max="key.startsWith('x') ? region?.logicalWidth : region?.logicalHeight" :step="1" :precision="1" :disabled="saving || busy || !region" :input-props="{ 'aria-label': key.toUpperCase() }" />
      </label>
    </div>
    <p v-if="modified" :class="{ invalid: !valid }">{{ valid ? '坐标尚未应用，预览仍显示当前区域。' : 'X2 必须大于 X1，Y2 必须大于 Y1，宽高至少为 4，且不能超出当前显示器。' }}</p>
    <NAlert v-if="error" type="error" :bordered="false">{{ error }}<NButton size="tiny" @click="preview.refresh">重试预览</NButton></NAlert>
    <footer>
      <NButton size="small" :disabled="!valid || !modified || busy || saving || transitioning" :loading="saving" @click="apply">应用坐标</NButton>
      <NButton size="small" :disabled="saving || busy || transitioning" @click="finish('reselect')">重新框选</NButton>
      <NButton size="small" type="primary" :disabled="!region || modified || saving || busy || !!error || transitioning" :loading="transitioning" @click="finish('translate')">翻译此区域</NButton>
    </footer>
  </section>
</template>

<style scoped>
.region-panel { display: grid; gap: 10px; }
header, footer { display: flex; align-items: center; justify-content: space-between; gap: 8px; }
header b { font-size: 13px; } header span, p { font-size: 11px; color: var(--muted); line-height: 1.5; }
.region-image { height: 160px; display: grid; place-items: center; overflow: hidden; border-radius: 8px; background: var(--surface-muted); }
.region-image img { width: 100%; height: 100%; object-fit: contain; }
.coordinates { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }
.coordinates label { display: grid; grid-template-columns: 25px minmax(0, 1fr); align-items: center; gap: 6px; font-size: 11px; color: var(--muted); }
.invalid { color: var(--danger, #ffb4b4); }
footer { flex-wrap: wrap; }
</style>
