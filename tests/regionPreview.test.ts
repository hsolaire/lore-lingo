import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { defineComponent } from 'vue'
import { flushPromises, mount } from '@vue/test-utils'
import { useRegionPreview } from '@/composables/useRegionPreview'
import type { RegionPreview } from '@/types/capturedLayout'

const client = vi.hoisted(() => ({ region: vi.fn(), regionPreview: vi.fn(), updateRegion: vi.fn() }))
vi.mock('@/services/captureTranslationClient', () => ({ captureTranslationClient: client }))
const selected = { revision: 1, monitorId: 'screen', logicalWidth: 1000, logicalHeight: 800, bounds: { x1: 10, x2: 110, y1: 20, y2: 120 } }
const frame = (revision = 1): RegionPreview => ({ revision, imagePngB64: 'cG5n', imageWidth: 200, imageHeight: 200, capturedAtMs: 1000 })
function harness() {
  let preview!: ReturnType<typeof useRegionPreview>
  const wrapper = mount(defineComponent({ setup() { preview = useRegionPreview(); return () => null } }))
  return { wrapper, get preview() { return preview } }
}

beforeEach(() => {
  vi.useFakeTimers()
  vi.resetAllMocks()
  client.region.mockResolvedValue(selected)
  client.regionPreview.mockResolvedValue(frame())
})
afterEach(() => { vi.useRealTimers() })

describe('实时区域预览', () => {
  it('慢帧不排队，完成后才刷新，关闭面板停止轮询', async () => {
    let resolve!: (value: RegionPreview) => void
    client.regionPreview.mockReturnValueOnce(new Promise<RegionPreview>(done => { resolve = done }))
    const current = harness()
    await flushPromises()
    await vi.advanceTimersByTimeAsync(3000)
    expect(client.regionPreview).toHaveBeenCalledTimes(1)
    resolve(frame())
    await flushPromises()
    await vi.advanceTimersByTimeAsync(500)
    expect(client.regionPreview).toHaveBeenCalledTimes(2)
    current.wrapper.unmount()
    await vi.advanceTimersByTimeAsync(2000)
    expect(client.regionPreview).toHaveBeenCalledTimes(2)
  })

  it('修改坐标等待旧帧结束并丢弃旧响应，使用新的修订号', async () => {
    let resolve!: (value: RegionPreview) => void
    client.regionPreview.mockReturnValueOnce(new Promise<RegionPreview>(done => { resolve = done })).mockResolvedValue(frame(2))
    const bounds = { ...selected.bounds, x1: 30 }
    client.updateRegion.mockResolvedValue({ ...selected, revision: 2, bounds })
    const current = harness()
    await flushPromises()
    const applying = current.preview.apply(bounds)
    expect(client.updateRegion).not.toHaveBeenCalled()
    resolve(frame())
    await applying
    expect(client.updateRegion).toHaveBeenCalledWith(1, bounds)
    expect(client.regionPreview).toHaveBeenLastCalledWith(2)
    expect(current.preview.frame.value?.revision).toBe(2)
    current.wrapper.unmount()
  })

  it('翻译前停止刷新并等待当前帧释放，避免捕获冲突', async () => {
    let resolve!: (value: RegionPreview) => void
    client.regionPreview.mockReturnValueOnce(new Promise<RegionPreview>(done => { resolve = done }))
    const current = harness()
    await flushPromises()
    let stopped = false
    const stopping = current.preview.stop().then(() => { stopped = true })
    expect(stopped).toBe(false)
    resolve(frame())
    await stopping
    expect(current.preview.frame.value).toBeNull()
    await vi.advanceTimersByTimeAsync(2000)
    expect(client.regionPreview).toHaveBeenCalledTimes(1)
    current.wrapper.unmount()
  })

  it('失败暂停并支持手动恢复，卸载后不更新画面', async () => {
    client.regionPreview.mockRejectedValueOnce({ message: '录屏权限被拒绝' })
    const current = harness()
    await flushPromises()
    expect(current.preview.error.value).toBe('录屏权限被拒绝')
    await vi.advanceTimersByTimeAsync(2000)
    expect(client.regionPreview).toHaveBeenCalledTimes(1)
    await current.preview.refresh()
    expect(current.preview.error.value).toBe('')
    let resolve!: (value: RegionPreview) => void
    client.regionPreview.mockReturnValueOnce(new Promise<RegionPreview>(done => { resolve = done }))
    await vi.advanceTimersByTimeAsync(500)
    current.wrapper.unmount()
    resolve(frame(100))
    await flushPromises()
    expect(current.preview.frame.value?.revision).toBe(1)
  })
})
