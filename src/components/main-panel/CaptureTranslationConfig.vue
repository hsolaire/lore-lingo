<script setup lang="ts">
import { reactive, watch } from 'vue'
import { NAlert, NButton, NForm, NFormItem, NInput, NModal } from 'naive-ui'

import type { TranslationConfig } from '@/types/capturedLayout'

const props = defineProps<{
  show: boolean
  config: Readonly<TranslationConfig>
}>()

const emit = defineEmits<{
  close: []
  save: [config: TranslationConfig]
}>()

const draft = reactive<TranslationConfig>({ ...props.config })

watch(
  () => props.show,
  show => {
    if (show) Object.assign(draft, props.config)
  },
)

function save() {
  emit('save', {
    endpoint: draft.endpoint.trim(),
    model: draft.model.trim(),
    apiKey: draft.apiKey.trim(),
    sourceLanguage: draft.sourceLanguage.trim() || 'auto',
    targetLanguage: draft.targetLanguage.trim() || 'zh-CN',
  })
}
</script>

<template>
  <NModal
    :show="show"
    preset="card"
    title="本次截图翻译配置"
    class="translation-config-modal"
    :mask-closable="false"
    @close="emit('close')"
    @esc="emit('close')"
  >
    <NForm label-placement="top" size="small" @submit.prevent="save">
      <NFormItem label="OpenAI 兼容端点">
        <NInput v-model:value="draft.endpoint" placeholder="https://api.example.com/v1" />
      </NFormItem>
      <NFormItem label="模型">
        <NInput v-model:value="draft.model" placeholder="例如 gpt-4.1-mini" />
      </NFormItem>
      <NFormItem label="API Key">
        <NInput
          v-model:value="draft.apiKey"
          type="password"
          show-password-on="click"
          placeholder="本地回环端点可以留空"
        />
      </NFormItem>
      <div class="language-grid">
        <NFormItem label="源语言">
          <NInput v-model:value="draft.sourceLanguage" placeholder="auto" />
        </NFormItem>
        <NFormItem label="目标语言">
          <NInput v-model:value="draft.targetLanguage" placeholder="zh-CN" />
        </NFormItem>
      </div>
      <NAlert type="info" :bordered="false">
        API Key、截图和识别文本只保留在当前应用进程内，不写入浏览器存储或 SQLite。
      </NAlert>
      <div class="actions">
        <NButton @click="emit('close')">取消</NButton>
        <NButton type="primary" attr-type="submit">应用配置</NButton>
      </div>
    </NForm>
  </NModal>
</template>

<style>
.translation-config-modal { width: min(520px, calc(100vw - 32px)); }
.translation-config-modal .language-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
.translation-config-modal .actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 18px; }
@media (max-width: 480px) { .translation-config-modal .language-grid { grid-template-columns: 1fr; gap: 0; } }
</style>
