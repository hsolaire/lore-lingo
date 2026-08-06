import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import OcrSettingsPanel from '@/components/settings/OcrSettingsPanel.vue'

const settings = {
  engine: 'system',
  apiBase: null,
  hasSecret: false,
  recognitionLanguages: ['en-US'],
  sampleFps: 2,
  minConfidence: 0.75,
  deduplicateFrames: true,
  options: {},
}

describe('OCR 设置面板', () => {
  it('在 macOS 上将系统引擎标识显示为原生 OCR 名称', () => {
    const wrapper = mount(OcrSettingsPanel, {
      props: {
        platform: 'macos',
        settings,
        systemAvailable: true,
      } as never,
    })

    expect(wrapper.text()).toContain('macOS Vision（系统内置）')
    expect(wrapper.text()).not.toContain('当前引擎system')
  })

  it('在 Windows 上将系统引擎标识显示为原生 OCR 名称', () => {
    const wrapper = mount(OcrSettingsPanel, {
      props: {
        platform: 'windows',
        settings,
        systemAvailable: true,
      } as never,
    })

    expect(wrapper.text()).toContain('Windows OCR（系统内置）')
    expect(wrapper.text()).not.toContain('当前引擎system')
  })
})
