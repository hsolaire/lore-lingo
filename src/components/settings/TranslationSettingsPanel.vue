<script setup lang="ts">
import { computed, ref, type DeepReadonly } from 'vue'
import { ChevronDown } from 'lucide-vue-next'
import { NButton, NIcon, NTag } from 'naive-ui'

import CaptureServiceSettings from './CaptureServiceSettings.vue'

import type { ModelProfileSummary, TranslationSettingsSummary } from '@/types/settings'

const props = defineProps<{
  settings: DeepReadonly<TranslationSettingsSummary> | null
  defaultModel: DeepReadonly<ModelProfileSummary> | null
}>()
const serviceStatus = ref<'loading' | 'configured' | 'unconfigured' | 'error'>('loading')
const showPlanned = ref(false)
const statusLabels = { loading: '正在读取', configured: '已应用', unconfigured: '待配置', error: '读取失败' } as const

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
</script>

<template>
  <section class="settings-section" aria-labelledby="translation-heading">
    <div class="section-heading">
      <div>
        <h2 id="translation-heading">翻译</h2>
        <p>为区域和窗口翻译配置同一个服务。</p>
      </div>
      <NTag size="small" :type="serviceStatus === 'configured' ? 'success' : serviceStatus === 'error' ? 'error' : 'default'" round role="status">
        {{ statusLabels[serviceStatus] }}
      </NTag>
    </div>

    <div class="service-heading"><h3>服务与模型</h3><span>单次翻译</span></div>
    <CaptureServiceSettings @status="serviceStatus = $event" />
    <p v-if="settings" class="language-note">启动默认语言：{{ sourceLanguage }} → {{ targetLanguage }}<span>可在悬浮工具栏的“翻译”菜单中切换。</span></p>

    <div class="planned-features">
      <NButton quaternary block class="planned-toggle" aria-controls="planned-features" :aria-expanded="showPlanned" @click="showPlanned = !showPlanned">
        <span>更多功能</span><span class="pending-label">尚未接入</span><NIcon :component="ChevronDown" :class="{ expanded: showPlanned }" />
      </NButton>
      <div id="planned-features" :hidden="!showPlanned">
        <ul class="planned-list">
          <li><b>自动翻译</b><span>按间隔持续识别和刷新译文</span></li>
          <li><b>字幕库匹配</b><span>优先使用已导入的本地字幕</span></li>
          <li><b>翻译历史</b><span>保存记录并设置保留时间</span></li>
          <li><b>重启后保留配置</b><span>自动恢复服务配置与安全保存的密钥</span></li>
        </ul>
      </div>
    </div>
  </section>
</template>

<style scoped>
.settings-section { display: flex; flex-direction: column; gap: 18px; }
.section-heading { display: flex; justify-content: space-between; align-items: flex-start; gap: 12px; padding-bottom: 18px; border-bottom: 1px solid var(--border); }
.section-heading h2 { font: 650 20px/1.2 var(--font-display); }
.section-heading p { margin-top: 6px; color: var(--muted); font-size: 12px; line-height: 1.5; }
.service-heading { display: flex; align-items: center; justify-content: space-between; }
.service-heading h3 { font: 600 13px/1.4 var(--font); }
.service-heading > span, .pending-label { font-size: 11px; color: var(--faint); }
.language-note { font-size: 11px; line-height: 1.6; color: var(--muted); }
.language-note span { display: block; color: var(--faint); }
.planned-features { border-top: 1px solid var(--border); padding-top: 10px; }
.planned-toggle :deep(.n-button__content) { width: 100%; gap: 8px; }
.pending-label { margin-left: auto; }
.planned-toggle :deep(.n-icon) { transition: transform .16s; }
.planned-toggle :deep(.expanded) { transform: rotate(180deg); }
.planned-list { list-style: none; display: grid; gap: 14px; padding: 16px 10px 0; }
.planned-list li { display: flex; flex-direction: column; gap: 3px; }
.planned-list b { font-size: 12px; font-weight: 550; color: var(--muted); }
.planned-list span { font-size: 11px; line-height: 1.5; color: var(--faint); }
</style>
