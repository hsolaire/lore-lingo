<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { Check, RotateCcw, ShieldCheck } from 'lucide-vue-next'
import { NAlert, NButton, NForm, NFormItem, NIcon, NInput, NSpin } from 'naive-ui'
import { captureTranslationClient } from '@/services/captureTranslationClient'
import type { TranslationConfig } from '@/types/capturedLayout'

type ConfigStatus = 'loading' | 'configured' | 'unconfigured' | 'error'
const emit = defineEmits<{ configured: [ready: boolean]; status: [status: ConfigStatus] }>()
const draft = reactive<TranslationConfig>({ endpoint: '', model: '', apiKey: '', sourceLanguage: 'auto', targetLanguage: 'zh-CN' })
const applied = ref<TranslationConfig | null>(null)
const loading = ref(true)
const saving = ref(false)
const loadFailed = ref(false)
const message = ref('')
const failed = ref(false)
const submitted = ref(false)
const dirty = computed(() => applied.value !== null && Object.keys(draft).some(key => draft[key as keyof TranslationConfig] !== applied.value?.[key as keyof TranslationConfig]))
const endpointError = computed(() => {
  if (!draft.endpoint.trim()) return '请输入翻译服务地址。'
  try {
    const url = new URL(draft.endpoint.trim())
    if (!['http:', 'https:'].includes(url.protocol) || !url.hostname) return '请输入完整的 HTTP 或 HTTPS 地址。'
    if (url.username || url.password || url.search || url.hash) return '地址不能包含用户名、密码、查询参数或片段。'
    return ''
  } catch { return '请输入有效的服务地址，例如 https://api.example.com/v1。' }
})
const localService = computed(() => {
  try {
    const host = new URL(draft.endpoint.trim()).hostname.toLowerCase()
    return host === 'localhost' || host === '[::1]' || /^127\.(\d{1,3}\.){2}\d{1,3}$/.test(host)
  } catch { return false }
})
const modelError = computed(() => !draft.model.trim() ? '请输入服务支持的模型名称。' : draft.model.trim().length > 256 ? '模型名称不能超过 256 个字符。' : '')
const keyError = computed(() => !localService.value && !draft.apiKey.trim() ? '远程服务需要填写 API Key。' : '')
const valid = computed(() => !endpointError.value && !modelError.value && !keyError.value)

async function load() {
  if (saving.value) return
  // 步骤 1. 读取期间关闭表单，避免空草稿覆盖已有配置。
  loading.value = true
  loadFailed.value = false
  message.value = ''
  emit('status', 'loading')
  try {
    // 步骤 2. 从原生进程恢复草稿及撤销基线，不写入浏览器存储。
    const config = await captureTranslationClient.getConfig()
    Object.assign(draft, config)
    applied.value = { ...config }
    emit('configured', valid.value)
    emit('status', valid.value ? 'configured' : 'unconfigured')
  } catch {
    // 步骤 3. 读取失败仅提供重试，不把默认空值当作有效配置。
    loadFailed.value = true
    emit('status', 'error')
  } finally { loading.value = false }
}

function reset() {
  if (applied.value) Object.assign(draft, applied.value)
  submitted.value = false
  message.value = ''
}

async function save() {
  // 步骤 1. 拦截未加载、重复提交和字段错误。
  if (loading.value || saving.value || loadFailed.value) return
  submitted.value = true
  if (!valid.value) return
  saving.value = true
  message.value = ''
  try {
    // 步骤 2. 交由后端按实际请求规则校验并应用运行期配置。
    const config = await captureTranslationClient.saveConfig({ ...draft, endpoint: draft.endpoint.trim(), model: draft.model.trim(), apiKey: draft.apiKey.trim() })
    // 步骤 3. 仅在成功后更新撤销基线；失败保留旧配置并显示安全提示。
    Object.assign(draft, config)
    applied.value = { ...config }
    emit('configured', true)
    emit('status', 'configured')
    failed.value = false
    message.value = '配置已应用，下一次捕获将使用此服务和模型。'
  } catch {
    failed.value = true
    message.value = '无法应用配置。请检查地址、模型和 API Key；当前使用的配置未被修改。'
  } finally { saving.value = false }
}

watch(draft, () => { message.value = '' }, { flush: 'sync' })
onMounted(load)
</script>

<template>
  <div class="capture-service">
    <div v-if="loading" class="config-loading" role="status"><NSpin size="small" /><span>正在读取翻译配置…</span></div>
    <NAlert v-else-if="loadFailed" type="error" :bordered="false" role="alert">
      无法读取翻译配置。重试成功后即可编辑。
      <div class="retry-action"><NButton size="small" @click="load">重新读取</NButton></div>
    </NAlert>
    <NForm v-else label-placement="top" size="small" class="capture-service-form" :disabled="saving" @submit.prevent="save">
      <NFormItem label="服务地址" :validation-status="submitted && endpointError ? 'error' : undefined">
        <NInput v-model:value="draft.endpoint" :input-props="{ 'aria-label': '翻译服务地址', 'aria-invalid': submitted && !!endpointError, 'aria-describedby': submitted && endpointError ? 'endpoint-feedback' : undefined, spellcheck: false }" placeholder="https://api.example.com/v1" />
        <template #feedback><span v-if="submitted && endpointError" id="endpoint-feedback">{{ endpointError }}</span></template>
      </NFormItem>
      <NFormItem label="模型名称" :validation-status="submitted && modelError ? 'error' : undefined">
        <NInput v-model:value="draft.model" :input-props="{ 'aria-label': '翻译模型', 'aria-invalid': submitted && !!modelError, 'aria-describedby': submitted && modelError ? 'model-feedback' : undefined, spellcheck: false }" placeholder="输入服务支持的模型名称" />
        <template #feedback><span v-if="submitted && modelError" id="model-feedback">{{ modelError }}</span></template>
      </NFormItem>
      <NFormItem label="API Key" :validation-status="submitted && keyError ? 'error' : undefined">
        <NInput v-model:value="draft.apiKey" :input-props="{ 'aria-label': 'API Key', 'aria-invalid': submitted && !!keyError, 'aria-describedby': submitted && keyError ? 'key-feedback' : undefined, autocomplete: 'off' }" type="password" show-password-on="click" :placeholder="localService ? '本地服务可留空' : '填写服务提供的 API Key'" />
        <template #feedback><span v-if="submitted && keyError" id="key-feedback">{{ keyError }}</span></template>
      </NFormItem>
      <div class="session-note"><NIcon :component="ShieldCheck" :size="15" /><p>仅在本次运行中生效，重启后需重新配置。API Key 不会写入磁盘。</p></div>
      <NAlert v-if="message" :type="failed ? 'error' : 'success'" :bordered="false" class="save-message" :role="failed ? 'alert' : 'status'">{{ message }}</NAlert>
      <footer class="form-actions">
        <span class="draft-status" role="status">{{ dirty ? '有未应用的修改' : valid ? '当前配置已应用' : '填写服务信息后应用' }}</span>
        <NButton v-if="dirty" quaternary :disabled="saving" @click="reset"><template #icon><NIcon :component="RotateCcw" /></template>撤销修改</NButton>
        <NButton type="primary" attr-type="submit" :loading="saving"><template #icon><NIcon :component="Check" /></template>应用配置</NButton>
      </footer>
    </NForm>
  </div>
</template>

<style scoped>
.capture-service { min-width: 0; }
.config-loading { display: flex; align-items: center; gap: 10px; min-height: 180px; justify-content: center; color: var(--muted); font-size: 12px; }
.capture-service-form :deep(.n-form-item-label) { font-weight: 550; color: var(--muted); }
.capture-service-form :deep(.n-input) { min-height: 34px; }
.session-note { display: flex; align-items: flex-start; gap: 7px; color: var(--faint); font-size: 11px; line-height: 1.6; margin-bottom: 16px; }
.session-note :deep(.n-icon) { margin-top: 2px; flex: none; }
.save-message { margin-bottom: 14px; }
.retry-action { margin-top: 10px; }
.form-actions { display: flex; align-items: center; justify-content: flex-end; gap: 8px; padding-top: 16px; border-top: 1px solid var(--border); }
.draft-status { margin-right: auto; color: var(--faint); font-size: 11px; }
@media (max-width: 440px) { .form-actions { flex-wrap: wrap; }.draft-status { width: 100%; } }
</style>
