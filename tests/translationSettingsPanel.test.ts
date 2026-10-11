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
  it('未完成的功能默认收起，并通过可聚焦按钮展开与收起', async () => {
    const wrapper = mount(TranslationSettingsPanel, { props: { settings: baseSettings, defaultModel: null } })
    const toggle = wrapper.get('button[aria-controls="planned-features"]')
    expect(toggle.attributes('aria-expanded')).toBe('false')
    expect(wrapper.get('#planned-features').isVisible()).toBe(false)
    await toggle.trigger('click')
    expect(toggle.attributes('aria-expanded')).toBe('true')
    expect(wrapper.get('#planned-features').isVisible()).toBe(true)
    expect(wrapper.get('#planned-features').text()).not.toContain('重启后保留配置')
    await toggle.trigger('click')
    expect(wrapper.get('#planned-features').isVisible()).toBe(false)
    wrapper.unmount()
  })
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
