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
    expect(wrapper.text()).toContain('配置已保存并应用')
    expect(wrapper.emitted('configured')).toEqual([[true], [true]])
    expect(storage).not.toHaveBeenCalled()
    storage.mockRestore()
    wrapper.unmount()
  })
  it('应用失败显示安全错误，不显示本次保存成功', async () => {
    saveConfig.mockRejectedValue({ message: '模型名称不能为空' })
    const wrapper = mount(CaptureServiceSettings)
    await flushPromises()
    await wrapper.get('form').trigger('submit')
    await flushPromises()
    expect(wrapper.text()).toContain('当前使用的配置未被修改')
    expect(wrapper.get('.save-message').text()).not.toContain('配置已保存并应用')
    wrapper.unmount()
  })

  it('保存后的配置再次打开时由原生服务恢复，持久化说明保持可见', async () => {
    const saved = { ...config, model: 'saved-model' }
    saveConfig.mockResolvedValueOnce(saved)
    const first = mount(CaptureServiceSettings)
    await flushPromises()
    await first.get('input[aria-label="翻译模型"]').setValue(saved.model)
    await first.get('form').trigger('submit')
    await flushPromises()
    expect(first.text()).toContain('重启后会自动恢复')
    first.unmount()
    getConfig.mockResolvedValueOnce(saved)
    const reopened = mount(CaptureServiceSettings)
    await flushPromises()
    expect(reopened.get('input[aria-label="翻译模型"]').element).toHaveProperty('value', saved.model)
    expect(reopened.text()).toContain('系统钥匙串或凭据管理器')
    expect(reopened.text()).not.toContain('仅在本次运行中生效')
    expect(reopened.emitted('status')?.at(-1)).toEqual(['configured'])
    expect(saveConfig).toHaveBeenCalledTimes(1)
    reopened.unmount()
  })

  it('配置加载失败时禁止保存，重试成功后恢复编辑', async () => {
    getConfig.mockRejectedValueOnce(new Error('内部诊断信息'))
    const wrapper = mount(CaptureServiceSettings)
    await flushPromises()
    expect(wrapper.find('form').exists()).toBe(false)
    expect(wrapper.text()).not.toContain('内部诊断信息')
    expect(wrapper.emitted('status')).toEqual([['loading'], ['error']])
    await wrapper.get('button').trigger('click')
    await flushPromises()
    expect(getConfig).toHaveBeenCalledTimes(2)
    expect(wrapper.find('form').exists()).toBe(true)
    expect(wrapper.emitted('status')?.at(-1)).toEqual(['configured'])
    expect(saveConfig).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it.each([
    { endpoint: 'file:///invalid', model: 'model', apiKey: '', feedback: 'HTTP 或 HTTPS' },
    { endpoint: 'https://api.example.com/v1?token=value', model: 'model', apiKey: '', feedback: '查询参数' },
    { endpoint: 'https://api.example.com/v1', model: 'model', apiKey: '', feedback: '需要填写 API Key' },
    { endpoint: 'http://127.0.0.1:11434/v1', model: '  ', apiKey: '', feedback: '请输入服务支持的模型名称' },
  ])('无效配置不发起保存：$feedback', async ({ endpoint, model, apiKey, feedback }) => {
    getConfig.mockResolvedValueOnce({ ...config, endpoint, model, apiKey })
    const wrapper = mount(CaptureServiceSettings)
    await flushPromises()
    await wrapper.get('form').trigger('submit')
    await flushPromises()
    expect(saveConfig).not.toHaveBeenCalled()
    expect(wrapper.text()).toContain(feedback)
    expect(wrapper.emitted('configured')).toEqual([[false]])
    wrapper.unmount()
  })

  it('可以撤销草稿，应用时去除空格且慢请求不会重复保存', async () => {
    let resolve!: (value: typeof config) => void
    saveConfig.mockReturnValueOnce(new Promise<typeof config>(done => { resolve = done }))
    const wrapper = mount(CaptureServiceSettings)
    await flushPromises()
    const model = wrapper.get('input[aria-label="翻译模型"]')
    await model.setValue('changed-model')
    expect(wrapper.text()).toContain('有未保存的修改')
    const reset = wrapper.findAll('button').find(button => button.text().includes('撤销修改'))!
    await reset.trigger('click')
    expect(model.element).toHaveProperty('value', 'test-model')
    await model.setValue(' new-model ')
    await wrapper.get('form').trigger('submit')
    await wrapper.get('form').trigger('submit')
    expect(saveConfig).toHaveBeenCalledTimes(1)
    expect(saveConfig).toHaveBeenCalledWith({ ...config, model: 'new-model' })
    resolve({ ...config, model: 'new-model' })
    await flushPromises()
    expect(model.element).toHaveProperty('value', 'new-model')
    expect(wrapper.text()).toContain('配置已保存并应用')
    expect(wrapper.text()).not.toContain('有未保存的修改')
    wrapper.unmount()
  })
})
