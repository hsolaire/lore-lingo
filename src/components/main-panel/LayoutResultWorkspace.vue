<script setup lang="ts">
import { computed, ref } from 'vue'
import { Languages, RotateCcw, SlidersHorizontal, X } from 'lucide-vue-next'
import { NButton, NEmpty, NSpin } from 'naive-ui'

import { useCaptureTranslation } from '@/composables/useCaptureTranslation'

type ResultMode = 'original' | 'translated' | 'bilingual'

const emit = defineEmits<{ configure: []; retry: []; close: [] }>()
const { state, cancel } = useCaptureTranslation()
const mode = ref<ResultMode>('translated')

const document = computed(() => state.value.document)
const imageUrl = computed(() => document.value ? `data:image/png;base64,${document.value.imagePngB64}` : '')
const active = computed(() => state.value.status !== 'idle' && state.value.status !== 'cancelled')

function blockStyle(bounds: { x: number; y: number; width: number; height: number }) {
  return {
    left: `${bounds.x * 100}%`,
    top: `${bounds.y * 100}%`,
    width: `${bounds.width * 100}%`,
    height: `${bounds.height * 100}%`,
  }
}

function closeWorkspace() {
  emit('close')
}
</script>

<template>
  <section v-if="active" class="result-workspace" aria-live="polite">
    <header class="result-toolbar">
      <div class="result-title">
        <Languages :size="16" />
        <span>截图翻译</span>
      </div>
      <div v-if="document && state.status === 'success'" class="mode-switch" role="tablist" aria-label="结果显示模式">
        <button v-for="item in (['original', 'translated', 'bilingual'] as ResultMode[])" :key="item" :class="{ on: mode === item }" role="tab" :aria-selected="mode === item" @click="mode = item">
          {{ item === 'original' ? '原图' : item === 'translated' ? '译文' : '双语' }}
        </button>
      </div>
      <div class="result-actions">
        <NButton quaternary circle aria-label="配置翻译服务" title="配置翻译服务" @click="emit('configure')">
          <template #icon><SlidersHorizontal /></template>
        </NButton>
        <NButton quaternary circle aria-label="关闭结果" title="关闭结果" @click="closeWorkspace">
          <template #icon><X /></template>
        </NButton>
      </div>
    </header>

    <div v-if="state.status === 'selecting'" class="result-state">
      <NSpin size="large" />
      <b>请选择屏幕区域</b>
      <p>在任意显示器上拖拽框选；按 Escape 可取消。</p>
      <NButton @click="cancel">取消框选</NButton>
    </div>

    <div v-else-if="state.status === 'processing'" class="result-state">
      <NSpin size="large" />
      <b>正在识别并翻译</b>
      <p>截图只在本次任务中使用，完成后不会持久化。</p>
    </div>

    <div v-else-if="state.status === 'error'" class="result-state error" role="alert">
      <img v-if="document" class="state-preview" :src="imageUrl" alt="已捕获的截图" />
      <b>{{ state.error.message }}</b>
      <p>错误代码：{{ state.error.code }}</p>
      <div class="state-actions">
        <NButton @click="emit('configure')">检查配置</NButton>
        <NButton v-if="state.error.recoverable" type="primary" @click="emit('retry')">
          <template #icon><RotateCcw /></template>
          重试
        </NButton>
      </div>
    </div>

    <div v-else-if="state.status === 'empty'" class="result-state">
      <img v-if="document" class="state-preview" :src="imageUrl" alt="已捕获的截图" />
      <NEmpty description="截图没有识别到可用文字" />
      <NButton type="primary" @click="emit('retry')">重新捕获</NButton>
    </div>

    <div v-else-if="document" class="canvas-scroll">
      <div class="layout-canvas" :style="{ aspectRatio: `${document.imageWidth} / ${document.imageHeight}` }">
        <img :src="imageUrl" alt="所选屏幕区域" />
        <template v-if="mode !== 'original'">
          <article
            v-for="block in document.blocks"
            :key="block.id"
            class="layout-block"
            :class="[`kind-${block.kind}`, { bilingual: mode === 'bilingual' }]"
            :style="blockStyle(block.bounds)"
          >
            <small v-if="mode === 'bilingual'">{{ block.sourceText }}</small>
            <span>{{ block.translatedText }}</span>
          </article>
        </template>
      </div>
    </div>
  </section>
</template>

<style scoped>
.result-workspace { position: absolute; inset: 0; z-index: 30; display: flex; flex-direction: column; min-width: 0; background: var(--glass-strong); backdrop-filter: blur(36px) saturate(1.4); }
.result-toolbar { min-height: 52px; display: flex; align-items: center; gap: 10px; padding: 8px 10px 8px 14px; border-bottom: 1px solid var(--border); }
.result-title { display: flex; align-items: center; gap: 7px; font-size: 12px; font-weight: 650; white-space: nowrap; }
.mode-switch { display: flex; gap: 2px; padding: 3px; margin: auto; border: 1px solid var(--border); border-radius: 9px; background: var(--surface-muted); }
.mode-switch button { border: 0; border-radius: 6px; padding: 6px 9px; background: transparent; color: var(--muted); font: 550 11px/1 var(--font); cursor: pointer; }
.mode-switch button.on { background: var(--glass-hi); color: var(--fg); box-shadow: 0 1px 4px #0004; }
.result-actions { display: flex; margin-left: auto; }
.result-state { flex: 1; display: grid; place-content: center; justify-items: center; gap: 12px; padding: 28px; text-align: center; }
.result-state b { font-size: 16px; }.result-state p { color: var(--muted); font-size: 12px; line-height: 1.55; }.result-state.error b { color: #ffb4b4; }
.state-actions { display: flex; gap: 8px; }
.state-preview { display: block; max-width: 100%; max-height: 180px; object-fit: contain; border-radius: 8px; }
.canvas-scroll { flex: 1; min-height: 0; padding: 12px; overflow: auto; }
.layout-canvas { position: relative; width: 100%; min-height: 160px; overflow: hidden; border: 1px solid var(--border-hi); border-radius: 11px; background: #08090c; box-shadow: 0 14px 40px #0007; }
.layout-canvas > img { position: absolute; inset: 0; width: 100%; height: 100%; object-fit: contain; display: block; }
.layout-block { position: absolute; display: flex; flex-direction: column; justify-content: center; gap: 2px; min-width: 24px; min-height: 18px; padding: clamp(2px, .45vw, 6px); overflow: auto; border: 1px solid color-mix(in oklch, var(--accent) 55%, transparent); border-radius: 5px; background: oklch(13% .025 255 / .9); color: #fff; font-size: clamp(8px, 1.45vw, 18px); line-height: 1.18; overflow-wrap: anywhere; box-shadow: 0 2px 10px #0007; }
.layout-block.kind-title { font-weight: 700; }.layout-block small { color: #bbc5cf; font-size: .7em; line-height: 1.1; }.layout-block.bilingual { justify-content: flex-start; }
button:focus-visible, :deep(button:focus-visible) { outline: 2px solid var(--focus); outline-offset: 2px; }
@media (max-width: 440px) { .result-title span { display: none; }.mode-switch button { padding-inline: 7px; }.canvas-scroll { padding: 8px; } }
</style>
