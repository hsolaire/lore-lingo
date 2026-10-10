<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { NAlert, NButton, NForm, NFormItem, NInput } from 'naive-ui'
import { captureTranslationClient } from '@/services/captureTranslationClient'
import type { TranslationConfig } from '@/types/capturedLayout'

const emit = defineEmits<{ configured: [ready: boolean] }>()
const draft = reactive<TranslationConfig>({ endpoint: '', model: '', apiKey: '', sourceLanguage: 'auto', targetLanguage: 'zh-CN' })
const loading = ref(true)
const saving = ref(false)
const message = ref('')
const failed = ref(false)

onMounted(async () => {
  try {
    Object.assign(draft, await captureTranslationClient.getConfig())
    emit('configured', !!draft.endpoint && !!draft.model)
  }
  catch { failed.value = true; message.value = '无法读取翻译配置，请重新打开设置。' }
  finally { loading.value = false }
})

async function save() {
  saving.value = true
  message.value = ''
  try {
    Object.assign(draft, await captureTranslationClient.saveConfig({ ...draft, endpoint: draft.endpoint.trim(), model: draft.model.trim(), apiKey: draft.apiKey.trim() }))
    emit('configured', !!draft.endpoint && !!draft.model)
    failed.value = false
    message.value = '配置已应用，下一次捕获将使用此服务和模型。'
  } catch (reason: unknown) {
    failed.value = true
    message.value = typeof reason === 'object' && reason && 'message' in reason ? String(reason.message) : '应用配置失败，请重试。'
  } finally { saving.value = false }
}
</script>

<template>
  <NForm label-placement="top" size="small" class="capture-service-form" :disabled="loading || saving" @submit.prevent="save">
    <NFormItem label="翻译服务地址"><NInput v-model:value="draft.endpoint" :input-props="{ 'aria-label': '翻译服务地址' }" placeholder="https://api.example.com/v1" /></NFormItem>
    <NFormItem label="翻译模型"><NInput v-model:value="draft.model" :input-props="{ 'aria-label': '翻译模型' }" placeholder="输入服务支持的模型名称" /></NFormItem>
    <NFormItem label="API Key"><NInput v-model:value="draft.apiKey" :input-props="{ 'aria-label': 'API Key' }" type="password" placeholder="本地回环服务可以留空" /></NFormItem>
    <p class="session-note">配置供区域和窗口翻译共用。当前仅保留在应用运行期间，密钥不会写入浏览器存储或数据库。</p>
    <NAlert v-if="message" :type="failed ? 'error' : 'success'" :bordered="false" class="save-message">{{ message }}</NAlert>
    <NButton type="primary" attr-type="submit" :disabled="loading" :loading="saving">应用翻译配置</NButton>
  </NForm>
</template>

<style scoped>
.capture-service-form { padding: 14px; border: 1px solid var(--border); border-radius: 11px; background: var(--surface-muted); }
.session-note { color: var(--muted); font-size: 11px; line-height: 1.6; margin-bottom: 12px; }
.save-message { margin-bottom: 12px; }
</style>
