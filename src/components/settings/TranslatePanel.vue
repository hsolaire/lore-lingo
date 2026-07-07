<script setup lang="ts">
import { ref } from 'vue'
import { useToast } from '@/composables/useToast'
import Icon from '@/components/ui/Icon.vue'
import SelectField from '@/components/ui/SelectField.vue'

const { toast } = useToast()

const provider = ref('OpenAI · GPT-4o mini')
const apiBase = ref('https://api.openai.com/v1')
const apiKey = ref('sk-live-9f2c8a71b4e0d3')
const keyVisible = ref(false)
const srcLang = ref('English')
const tgtLang = ref('简体中文')

const status = ref<'ok' | 'testing'>('ok')

function testConnect() {
  status.value = 'testing'
  setTimeout(() => {
    status.value = 'ok'
    toast('连接正常')
  }, 1100)
}
</script>

<template>
  <div class="panel">
    <div class="field">
      <label>翻译服务商</label>
      <select class="select" v-model="provider">
        <option>OpenAI · GPT-4o mini</option>
        <option>Anthropic · Claude Haiku</option>
        <option>DeepL API</option>
        <option>Google 翻译 API</option>
        <option>本地 Ollama · qwen2.5</option>
        <option>自定义 OpenAI 兼容端点</option>
      </select>
    </div>
    <div class="field">
      <label>API Endpoint</label>
      <input class="input mono" v-model="apiBase" spellcheck="false" />
    </div>
    <div class="field">
      <label>API Key</label>
      <div class="key-wrap">
        <input
          class="input mono"
          v-model="apiKey"
          :type="keyVisible ? 'text' : 'password'"
          spellcheck="false"
        />
        <button class="eye" aria-label="显示/隐藏" @click="keyVisible = !keyVisible">
          <Icon name="eye" />
        </button>
      </div>
    </div>
    <div class="row2">
      <SelectField label="源语言" v-model="srcLang" :options="['自动检测', 'English', '日本語', '한국어']" />
      <SelectField label="目标语言" v-model="tgtLang" :options="['简体中文', '繁體中文', 'English']" />
    </div>
    <button class="btn test-btn" @click="testConnect">
      <Icon name="checkCircle" />
      测试连接
    </button>
    <span class="status-line" :class="{ warn: status === 'testing' }">
      <span class="dot"></span>
      <template v-if="status === 'testing'">正在测试…</template>
      <template v-else>已连接 · <b>GPT-4o mini</b> · 延迟 240ms</template>
    </span>
  </div>
</template>

<style scoped>
.panel { display: flex; flex-direction: column; gap: 14px; padding-top: 4px; animation: fade .25s ease; }

.field { display: flex; flex-direction: column; gap: 6px; }
.field > label { font: 500 11px/1 var(--font); color: var(--muted); letter-spacing: 0.01em; }
.input, .select {
  width: 100%; background: oklch(18% 0.02 260 / 0.6); color: var(--fg);
  border: 1px solid var(--border); border-radius: 9px; padding: 10px 12px;
  font: 400 13px/1.2 var(--font); transition: .15s; outline: none;
}
.input:focus, .select:focus { border-color: var(--accent); box-shadow: 0 0 0 3px var(--accent-dim); }
.input.mono { font-family: var(--mono); font-size: 12px; letter-spacing: 0; }
.select { appearance: none; cursor: pointer;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2399a' stroke-width='2.5'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat; background-position: right 12px center; padding-right: 34px; }
.row2 { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }

.key-wrap { position: relative; }
.key-wrap .eye { position: absolute; right: 8px; top: 50%; transform: translateY(-50%);
  width: 26px; height: 26px; border: 0; background: transparent; color: var(--faint); cursor: pointer;
  display: grid; place-items: center; border-radius: 6px; }
.key-wrap .eye:hover { color: var(--fg); }
.key-wrap .eye :deep(svg) { width: 15px; height: 15px; }
.key-wrap .input { padding-right: 38px; }

.status-line { display: inline-flex; align-items: center; gap: 6px; font: 400 11px/1 var(--font); color: var(--faint); }
.status-line b { font-weight: 550; }
.status-line .dot { width: 7px; height: 7px; border-radius: 50%; background: var(--ok); }
.status-line.warn .dot { background: var(--warn); }
.test-btn { align-self: flex-start; }

.btn {
  border: 1px solid var(--border); background: var(--glass-hi); color: var(--fg);
  border-radius: 9px; padding: 9px 14px; font: 500 12px/1 var(--font); cursor: pointer;
  transition: .15s; display: inline-flex; align-items: center; gap: 7px;
}
.btn:hover { border-color: var(--border-hi); background: oklch(72% 0.02 260 / 0.16); }
.btn :deep(svg) { width: 14px; height: 14px; }
</style>
