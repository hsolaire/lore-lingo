import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import TranslationSettingsPanel from '@/components/settings/TranslationSettingsPanel.vue'

const baseSettings = {
  defaultModelProfileId: null,
  mode: 'auto' as const,
  autoIntervalMs: 1000,
  sourceLanguage: 'en',
  targetLanguage: 'zh-CN',
  requestTimeoutMs: 30000,
  preferSubtitles: true,
  saveHistory: true,
  historyRetentionDays: null,
}

describe('翻译设置面板', () => {
  it('将数据库语言代码转换为面向用户的语言名称', () => {
    const wrapper = mount(TranslationSettingsPanel, {
      props: {
        settings: baseSettings,
        defaultModel: null,
      },
    })

    expect(wrapper.text()).toContain('English')
    expect(wrapper.text()).toContain('简体中文')
    expect(wrapper.text()).not.toMatch(/>en</)
    expect(wrapper.text()).not.toContain('zh-CN')
  })

  it('对未知语言代码保留原值，避免显示错误语言名称', () => {
    const wrapper = mount(TranslationSettingsPanel, {
      props: {
        settings: {
          ...baseSettings,
          sourceLanguage: 'eo',
          targetLanguage: 'yue-Hant',
        },
        defaultModel: null,
      },
    })

    expect(wrapper.text()).toContain('eo')
    expect(wrapper.text()).toContain('yue-Hant')
  })
})
