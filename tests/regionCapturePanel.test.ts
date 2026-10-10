import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import RegionCapturePanel from '@/components/main-panel/RegionCapturePanel.vue'
const client = vi.hoisted(() => ({ region: vi.fn(), regionPreview: vi.fn(), updateRegion: vi.fn() }))
vi.mock('@/services/captureTranslationClient', () => ({ captureTranslationClient: client }))
const region = { revision: 1, monitorId: 'screen', logicalWidth: 1000, logicalHeight: 800, bounds: { x1: 10, x2: 110, y1: 20, y2: 120 } }
beforeEach(() => {
  vi.useFakeTimers()
  vi.resetAllMocks()
  client.region.mockResolvedValue(region)
  client.regionPreview.mockImplementation((revision: number) => Promise.resolve({ revision, imagePngB64: 'cG5n', imageWidth: 200, imageHeight: 200, capturedAtMs: 1000 }))
})
afterEach(() => vi.useRealTimers())
describe('区域坐标面板', () => {
  it('无效坐标禁用应用；有效坐标提交后才允许翻译', async () => {
    const wrapper = mount(RegionCapturePanel, { props: { busy: false } })
    await flushPromises()
    const button = (text: string) => wrapper.findAll('button').find(item => item.text() === text)!
    const input = wrapper.get('input[aria-label="X2"]')
    await input.setValue('5')
    await input.trigger('blur')
    expect(wrapper.text()).toContain('X2 必须大于 X1')
    expect(button('应用坐标').attributes('disabled')).toBeDefined()
    await input.setValue('150')
    await input.trigger('blur')
    expect(button('应用坐标').attributes('disabled')).toBeUndefined()
    expect(button('翻译此区域').attributes('disabled')).toBeDefined()
    client.updateRegion.mockResolvedValue({ ...region, revision: 2, bounds: { ...region.bounds, x2: 150 } })
    await button('应用坐标').trigger('click')
    await flushPromises()
    expect(client.updateRegion).toHaveBeenCalledWith(1, { ...region.bounds, x2: 150 })
    await button('翻译此区域').trigger('click')
    await flushPromises()
    expect(wrapper.emitted('translate')).toEqual([[2]])
    await vi.advanceTimersByTimeAsync(2000)
    expect(client.regionPreview).toHaveBeenCalledTimes(2)
    wrapper.unmount()
  })
})
