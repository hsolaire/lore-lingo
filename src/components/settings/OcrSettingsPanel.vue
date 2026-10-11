<script setup lang="ts">
import { computed, type DeepReadonly } from 'vue'
import { CircleAlert, ScanLine } from 'lucide-vue-next'
import { NIcon, NProgress, NTag } from 'naive-ui'

import type { PlatformKind } from '@/types/desktop'
import type { OcrSettingsSummary } from '@/types/settings'

const props = withDefaults(defineProps<{
  settings: DeepReadonly<OcrSettingsSummary> | null
  systemAvailable: boolean
  platform?: PlatformKind
}>(), {
  platform: 'unsupported',
})

const confidencePercent = computed(() => Math.round((props.settings?.minConfidence ?? 0) * 100))
const engineName = computed(() => {
  const engine = props.settings?.engine
  if (!engine) return '尚未读取'
  if (engine === 'system') {
    if (props.platform === 'macos') return 'macOS Vision（系统内置）'
    if (props.platform === 'windows') return 'Windows OCR（系统内置）'
    return '系统内置 OCR'
  }
  if (engine === 'tesseract') return 'Tesseract（本地）'
  if (engine === 'paddleocr') return 'PaddleOCR（本地）'
  if (engine === 'cloud') return '云端 OCR'
  return '未识别的 OCR 引擎'
})
</script>

<template>
  <section class="settings-section" aria-labelledby="ocr-heading">
    <div class="section-heading">
      <div>
        <h2 id="ocr-heading">OCR</h2>
        <p>查看识别引擎配置与系统支持情况。</p>
      </div>
      <NTag size="small" :type="systemAvailable ? 'success' : 'warning'" round>
        {{ systemAvailable ? '系统 OCR 可用' : '系统 OCR 不可用' }}
      </NTag>
    </div>

    <div class="ocr-card">
      <NIcon :component="ScanLine" size="23" />
      <div>
        <span>已保存的引擎配置</span>
        <b>{{ engineName }}</b>
        <small>{{ settings?.recognitionLanguages.join('、') || '未设置识别语言' }}</small>
      </div>
    </div>

    <p class="preset-note">以下为连续识别预设，尚未用于当前单次捕获。</p>
    <div class="metric-grid">
      <div><span>采样帧率</span><b>{{ settings?.sampleFps ?? 0 }} fps</b></div>
      <div><span>连续帧去重</span><b>{{ settings?.deduplicateFrames ? '已开启' : '已关闭' }}</b></div>
    </div>

    <div class="confidence">
      <div><span>最低置信度</span><b>{{ confidencePercent }}%</b></div>
      <NProgress type="line" :percentage="confidencePercent" :show-indicator="false" />
    </div>

    <div class="honest-state" role="status">
      <NIcon :component="CircleAlert" size="17" />
      <span>{{ systemAvailable ? '当前平台支持系统 OCR。' : '当前平台暂未接入系统 OCR。' }}引擎切换、识别预设编辑与连续采样尚未接入。</span>
    </div>
  </section>
</template>

<style scoped>
.settings-section { display: flex; flex-direction: column; gap: 18px; }
.section-heading { display: flex; justify-content: space-between; align-items: flex-start; gap: 12px; }
.section-heading h2 { font: 650 19px/1.2 var(--font-display); }
.section-heading p { margin-top: 5px; color: var(--faint); font-size: 12px; }
.ocr-card { display: grid; grid-template-columns: 30px 1fr; gap: 11px; align-items: center; padding: 14px; border: 1px solid var(--border); border-radius: 11px; background: var(--surface-muted); }
.ocr-card > div { display: flex; flex-direction: column; gap: 4px; }
.ocr-card span, .metric-grid span, .confidence span { color: var(--faint); font-size: 10px; }
.ocr-card b, .metric-grid b { font-size: 13px; }
.ocr-card small { color: var(--muted); font-size: 11px; }
.preset-note { color: var(--faint); font-size: 11px; line-height: 1.5; }
.metric-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }
.metric-grid > div { display: flex; flex-direction: column; gap: 5px; padding: 12px; border: 1px solid var(--border); border-radius: 10px; }
.confidence { display: flex; flex-direction: column; gap: 8px; }
.confidence > div { display: flex; justify-content: space-between; align-items: center; }
.confidence b { font: 600 11px/1 var(--mono); }
.honest-state { display: flex; gap: 9px; padding: 12px; border: 1px solid color-mix(in oklch, var(--warn) 30%, transparent); border-radius: 10px; background: color-mix(in oklch, var(--warn) 10%, transparent); color: var(--muted); font-size: 11px; line-height: 1.5; }
</style>
