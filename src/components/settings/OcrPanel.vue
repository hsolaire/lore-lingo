<script setup lang="ts">
import { ref } from 'vue'
import ToggleSwitch from '@/components/ui/ToggleSwitch.vue'
import SelectField from '@/components/ui/SelectField.vue'

const engine = ref('macOS Vision（系统内置 · 离线）')
const recogLang = ref('英文')
const fps = ref('2 fps · 均衡')
const confidence = ref('0.75')
const dedupe = ref(true)
</script>

<template>
  <div class="panel">
    <div class="field">
      <label>屏幕 OCR 引擎</label>
      <select class="select" v-model="engine">
        <option>macOS Vision（系统内置 · 离线）</option>
        <option>Tesseract（本地）</option>
        <option>PaddleOCR（本地）</option>
        <option>云端 OCR · GPT-4o Vision</option>
      </select>
    </div>
    <SelectField label="识别语言" v-model="recogLang" :options="['英文', '英文 + 日文', '多语言']" />
    <div class="row2">
      <SelectField label="采样帧率" v-model="fps" :options="['1 fps · 省电', '2 fps · 均衡', '4 fps · 灵敏']" />
      <SelectField label="最低置信度" v-model="confidence" :options="['0.6', '0.75', '0.9 · 严格']" />
    </div>
    <div class="field">
      <ToggleSwitch v-model="dedupe" start>去重连续相同帧</ToggleSwitch>
      <span class="hint">同一句话在画面停留时只翻译一次，减少模型调用。</span>
    </div>
    <span class="status-line"><span class="dot"></span>Vision 引擎就绪 · 离线运行</span>
  </div>
</template>

<style scoped>
.panel { display: flex; flex-direction: column; gap: 14px; padding-top: 4px; animation: fade .25s ease; }

.field { display: flex; flex-direction: column; gap: 6px; }
.field > label { font: 500 11px/1 var(--font); color: var(--muted); letter-spacing: 0.01em; }
.hint { font: 400 11px/1.4 var(--font); color: var(--faint); }
.select {
  width: 100%; background: oklch(18% 0.02 260 / 0.6); color: var(--fg);
  border: 1px solid var(--border); border-radius: 9px; padding: 10px 12px;
  font: 400 13px/1.2 var(--font); transition: .15s; outline: none;
  appearance: none; cursor: pointer; padding-right: 34px;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2399a' stroke-width='2.5'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat; background-position: right 12px center;
}
.select:focus { border-color: var(--accent); box-shadow: 0 0 0 3px var(--accent-dim); }
.row2 { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }

.status-line { display: inline-flex; align-items: center; gap: 6px; font: 400 11px/1 var(--font); color: var(--faint); }
.status-line .dot { width: 7px; height: 7px; border-radius: 50%; background: var(--ok); }
</style>
