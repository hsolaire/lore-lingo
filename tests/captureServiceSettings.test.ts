import { flushPromises, mount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import CaptureServiceSettings from '@/components/settings/CaptureServiceSettings.vue'

const { getConfig, saveConfig } = vi.hoisted(() => ({ getConfig: vi.fn(), saveConfig: vi.fn() }))
vi.mock('@/services/captureTranslationClient', () => ({ captureTranslationClient: { getConfig, saveConfig } }))
const config = { endpoint: 'http://127.0.0.1:11434/v1', model: 'test-model', apiKey: '', sourceLanguage: 'auto', targetLanguage: 'zh-CN' }

describe('设置页共用翻译配置', () => {
  beforeEach(() => { vi.clearAllMocks(); getConfig.mockResolvedValue(config); saveConfig.mockResolvedValue(config) })
  it('从原生进程读取配置并应用到同一个服务，不写浏览器存储', async () => {
    const storage = vi.spyOn(Storage.prototype, 'setItem')
    const wrapper = mount(CaptureServiceSettings)
    await flushPromises()
    expect(wrapper.get('input[aria-label="翻译模型"]').element).toHaveProperty('value', 'test-model')
    expect(wrapper.emitted('configured')).toEqual([[true]])
    await wrapper.get('form').trigger('submit')
    await flushPromises()
    expect(saveConfig).toHaveBeenCalledWith(config)
    expect(wrapper.text()).toContain('配置已应用')
    expect(wrapper.emitted('configured')).toEqual([[true], [true]])
    expect(storage).not.toHaveBeenCalled()
    storage.mockRestore()
    wrapper.unmount()
  })
  it('应用失败显示真实错误，不显示成功', async () => {
    saveConfig.mockRejectedValue({ message: '模型名称不能为空' })
    const wrapper = mount(CaptureServiceSettings)
    await flushPromises()
    await wrapper.get('form').trigger('submit')
    await flushPromises()
    expect(wrapper.text()).toContain('模型名称不能为空')
    expect(wrapper.text()).not.toContain('配置已应用')
    wrapper.unmount()
  })
})
