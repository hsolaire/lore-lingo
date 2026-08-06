<script setup lang="ts">
import { computed, type DeepReadonly } from 'vue'
import {
  NAlert,
  NInputNumber,
  NRadio,
  NRadioGroup,
  NSelect,
  NSwitch,
  NTag,
} from 'naive-ui'

import type { ModelProfileSummary, TranslationSettingsSummary } from '@/types/settings'

const props = defineProps<{
  settings: DeepReadonly<TranslationSettingsSummary> | null
  defaultModel: DeepReadonly<ModelProfileSummary> | null
}>()

const languageNames: Record<string, string> = {
  auto: '自动检测',
  en: 'English',
  'en-US': 'English（美国）',
  'en-GB': 'English（英国）',
  ja: '日本語',
  ko: '한국어',
  zh: '中文',
  'zh-CN': '简体中文',
  'zh-Hans': '简体中文',
  'zh-TW': '繁體中文',
  'zh-Hant': '繁體中文',
}

function formatLanguageName(languageCode: string | undefined): string {
  if (!languageCode) return '尚未读取'
  return languageNames[languageCode] ?? languageCode
}

const sourceLanguageCode = computed(() => props.settings?.sourceLanguage ?? '')
const targetLanguageCode = computed(() => props.settings?.targetLanguage ?? '')
const sourceLanguage = computed(() => formatLanguageName(sourceLanguageCode.value))
const targetLanguage = computed(() => formatLanguageName(targetLanguageCode.value))
const intervalSeconds = computed(() => (props.settings?.autoIntervalMs ?? 1000) / 1000)
const timeoutSeconds = computed(() => (props.settings?.requestTimeoutMs ?? 30000) / 1000)
const languageOptions = computed(() => [
  { label: sourceLanguage.value, value: sourceLanguageCode.value },
  { label: targetLanguage.value, value: targetLanguageCode.value },
])
</script>

<template>
  <section class="settings-section" aria-labelledby="translation-heading">
    <div class="section-heading">
      <div>
        <h2 id="translation-heading">翻译</h2>
        <p>查看当前翻译方式、语言和模型配置。</p>
      </div>
      <NTag size="small" :type="defaultModel ? 'success' : 'warning'" round>
        {{ defaultModel ? '已配置模型' : '未配置模型' }}
      </NTag>
    </div>

    <div class="field-group">
      <span class="field-label">翻译方式</span>
      <NRadioGroup :value="settings?.mode ?? 'manual'" disabled name="translation-mode">
        <div class="choice-grid">
          <label class="choice-card">
            <NRadio value="manual" />
            <span><b>手动翻译</b><small>按快捷键后翻译当前画面一次</small></span>
          </label>
          <label class="choice-card">
            <NRadio value="auto" />
            <span><b>自动翻译</b><small>按固定间隔识别并刷新译文</small></span>
          </label>
        </div>
      </NRadioGroup>
    </div>

    <div class="form-grid">
      <label class="field-group">
        <span class="field-label">自动翻译间隔</span>
        <NInputNumber :value="intervalSeconds" disabled :show-button="false">
          <template #suffix>秒</template>
        </NInputNumber>
      </label>
      <label class="field-group">
        <span class="field-label">请求超时</span>
        <NInputNumber :value="timeoutSeconds" disabled :show-button="false">
          <template #suffix>秒</template>
        </NInputNumber>
      </label>
      <label class="field-group">
        <span class="field-label">源语言</span>
        <NSelect :value="sourceLanguageCode" :options="languageOptions" disabled />
      </label>
      <label class="field-group">
        <span class="field-label">目标语言</span>
        <NSelect :value="targetLanguageCode" :options="languageOptions" disabled />
      </label>
    </div>

    <div class="model-card">
      <div>
        <span class="field-label">默认翻译模型</span>
        <b>{{ defaultModel?.name ?? '尚未配置翻译模型' }}</b>
        <small v-if="defaultModel">{{ defaultModel.provider }} · {{ defaultModel.modelName }}</small>
        <small v-else>配置模型与系统凭据后才能发起真实翻译。</small>
      </div>
      <NTag size="small" :type="defaultModel?.hasSecret ? 'success' : 'default'">
        {{ defaultModel?.hasSecret ? '凭据已配置' : '无可用凭据' }}
      </NTag>
    </div>

    <div class="switch-list">
      <label><NSwitch :value="settings?.preferSubtitles ?? false" disabled />优先匹配字幕库</label>
      <label><NSwitch :value="settings?.saveHistory ?? false" disabled />保存翻译历史</label>
    </div>

    <NAlert type="info" :show-icon="true" :bordered="false">
      当前版本从本地数据库读取设置；编辑、凭据保存和连接测试尚未接入，因此不会显示模拟连接成功。
    </NAlert>
  </section>
</template>

<style scoped>
.settings-section { display: flex; flex-direction: column; gap: 18px; }
.section-heading { display: flex; justify-content: space-between; align-items: flex-start; gap: 12px; }
.section-heading h2 { font: 650 19px/1.2 var(--font-display); }
.section-heading p { margin-top: 5px; color: var(--faint); font-size: 12px; line-height: 1.45; }
.field-group { display: flex; flex-direction: column; gap: 8px; }
.field-label { color: var(--muted); font-size: 11px; font-weight: 550; }
.choice-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }
.choice-card { display: flex; gap: 9px; min-height: 72px; padding: 12px; border: 1px solid var(--border); border-radius: 11px; background: var(--surface-muted); }
.choice-card span { display: flex; flex-direction: column; gap: 5px; }
.choice-card b { font-size: 12px; }
.choice-card small, .model-card small { color: var(--faint); font-size: 10px; line-height: 1.45; }
.form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 13px 12px; }
.model-card { display: flex; justify-content: space-between; align-items: center; gap: 12px; padding: 13px; border: 1px solid var(--border); border-radius: 11px; background: var(--surface-muted); }
.model-card > div { display: flex; flex-direction: column; gap: 4px; min-width: 0; }
.model-card b { font-size: 13px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.switch-list { display: flex; flex-wrap: wrap; gap: 14px 20px; }
.switch-list label { display: inline-flex; align-items: center; gap: 8px; color: var(--muted); font-size: 12px; }
@media (max-width: 440px) { .choice-grid, .form-grid { grid-template-columns: 1fr; } }
</style>
