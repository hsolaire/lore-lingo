import { mount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { computed, ref } from 'vue'
import type { LayoutDocument } from '@/types/capturedLayout'
import type { CaptureSourceSummary } from '@/types/desktop'

const { openSettings, setMainControlsVisible, start, startWindow, fetchWindows, cancel } = vi.hoisted(() => ({
  openSettings: vi.fn<() => Promise<void>>(), setMainControlsVisible: vi.fn<() => Promise<void>>(),
  start: vi.fn<() => Promise<void>>(), cancel: vi.fn<() => Promise<void>>(),
  startWindow: vi.fn(), fetchWindows: vi.fn(),
}))
vi.mock('@/services/windowClient', () => ({ windowClient: { setMainControlsVisible } }))
vi.mock('@/composables/useWindowNavigation', () => ({ useWindowNavigation: () => ({ openSettings }) }))
vi.mock('@/composables/useSubtitleWindow', () => ({ useSubtitleWindow: () => ({
  pinned: ref(true), togglePinned: vi.fn(), onDragBarMousedown: vi.fn(), onResizeMousedown: vi.fn(),
}) }))
const sourceMode = ref('app')
const state = ref<{ status: string; document?: LayoutDocument }>({ status: 'idle' })
const windowList = ref<CaptureSourceSummary[]>([])
vi.mock('@/composables/useCaptureSource', () => ({ useCaptureSource: () => ({ sourceMode, targetName: ref('未选择窗口'), windowList, selectedWindow: ref(null), listLoading: ref(false), fetchWindows, selectWindow: vi.fn() }) }))
vi.mock('@/composables/useCaptureTranslation', () => ({ useCaptureTranslation: () => ({
  state, busy: computed(() => state.value.status === 'processing'),
  config: ref({ endpoint: '', model: '', apiKey: '', sourceLanguage: 'auto', targetLanguage: 'zh-CN' }),
  start, startWindow, cancel, setConfig: vi.fn(),
}) }))
vi.mock('@/composables/useTranslator', () => ({ useTranslator: () => ({ srcText: ref(''), tgtText: ref(''), langA: ref('auto'), langB: ref('zh-CN'), flip: vi.fn() }) }))

import MainPanel from '@/components/main-panel/MainPanel.vue'

function createOverlay(attached = false) {
  return mount(MainPanel, {
    props: { platform: 'macos' },
    ...(attached ? { attachTo: document.body } : {}),
    global: { stubs: {
      SubtitleControls: { template: `<header><button aria-label="编辑捕获区域" @focus="$emit('preview', true)" @click="$emit('translate')" /><button aria-label="翻译" @click="$emit('translate')" /><button aria-label="设置" @click="$emit('settings')" /><button aria-label="翻译选项" @click="$emit('options')" /><button aria-label="捕获窗口" @click="$emit('source')" /></header>`, emits: ['translate', 'settings', 'options', 'source', 'preview'] },
      SubtitleContent: { template: '<section data-testid="subtitle-content" />' },
      RegionCapturePanel: { template: '<input aria-label="测试坐标" />' }, SourceRow: true, CaptureTarget: true, LayoutResultWorkspace: true, CaptureTranslationConfig: true,
    } },
  })
}

describe('单窗口悬浮字幕壳', () => {
  beforeEach(() => {
    vi.useFakeTimers()
    localStorage.clear()
    vi.clearAllMocks()
    state.value = { status: 'idle' }
    sourceMode.value = 'app'
    windowList.value = []
    setMainControlsVisible.mockResolvedValue(undefined)
  })

  it('工具栏收起时也隐藏原生交通灯，字幕内容保留', async () => {
    const wrapper = createOverlay()
    await vi.advanceTimersByTimeAsync(2200)
    expect(wrapper.classes()).not.toContain('toolbar-visible')
    expect(setMainControlsVisible).toHaveBeenLastCalledWith(false)
    expect(wrapper.find('[data-testid="subtitle-content"]').exists()).toBe(true)
    await wrapper.trigger('pointerenter')
    expect(wrapper.classes()).toContain('toolbar-visible')
    expect(setMainControlsVisible).toHaveBeenLastCalledWith(true)
    wrapper.unmount()
    vi.useRealTimers()
  })

  it('展开捕获来源时不会因鼠标移开而收起工具栏', async () => {
    const wrapper = createOverlay()
    await wrapper.get('button[aria-label="捕获窗口"]').trigger('click')
    await wrapper.trigger('pointerleave')
    await vi.advanceTimersByTimeAsync(3000)
    expect(wrapper.classes()).toContain('toolbar-visible')
    await wrapper.trigger('keydown', { key: 'Escape' })
    await vi.advanceTimersByTimeAsync(450)
    expect(wrapper.classes()).not.toContain('toolbar-visible')
    wrapper.unmount()
    vi.useRealTimers()
  })

  it('选中窗口立即发起真实窗口任务，并关闭来源菜单', async () => {
    const source: CaptureSourceSummary = { sourceId: 'opaque-window', catalogRevision: 7, kind: 'window', title: '测试窗口', applicationName: '测试应用', iconDataUrl: null, monitor: null, capturable: true }
    windowList.value = [source]
    const wrapper = createOverlay()
    await wrapper.get('button[aria-label="捕获窗口"]').trigger('click')
    expect(fetchWindows).toHaveBeenCalledOnce()
    await wrapper.get('.window-list button').trigger('click')
    expect(startWindow).toHaveBeenCalledWith(source)
    expect(sourceMode.value).toBe('app')
    expect(wrapper.find('[aria-label="选择捕获窗口"]').exists()).toBe(false)
    wrapper.unmount()
    vi.useRealTimers()
  })

  it('翻译和来源弹层不扩张字幕表面或设置固定窗口高度', async () => {
    const wrapper = createOverlay()
    const surface = wrapper.get('.unified-overlay')
    const before = surface.get('.subtitle-surface').html()
    await wrapper.get('button[aria-label="翻译选项"]').trigger('click')
    expect(surface.get('.subtitle-surface').html()).toBe(before)
    expect(surface.attributes('style')).toBeUndefined()
    expect(wrapper.get('[aria-label="翻译选项"][role="dialog"]').element.parentElement).toBe(wrapper.element)
    expect(wrapper.attributes('style')).toBeUndefined()
    await wrapper.get('button[aria-label="捕获窗口"]').trigger('click')
    expect(surface.get('.subtitle-surface').html()).toBe(before)
    expect(surface.attributes('style')).toBeUndefined()
    expect(wrapper.get('[aria-label="选择捕获窗口"]').element.parentElement).toBe(wrapper.element)
    wrapper.unmount()
    vi.useRealTimers()
  })

  it('Escape 关闭翻译选项并把焦点返回入口', async () => {
    const wrapper = createOverlay()
    await wrapper.get('button[aria-label="翻译选项"]').trigger('click')
    expect(wrapper.get('[role="dialog"]').attributes('aria-label')).toBe('翻译选项')
    const focus = vi.spyOn(wrapper.get('button[aria-label="翻译选项"]').element as HTMLButtonElement, 'focus')
    await wrapper.get('[role="dialog"]').trigger('keydown', { key: 'Escape' })
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false)
    expect(focus).toHaveBeenCalledOnce()
    wrapper.unmount()
    vi.useRealTimers()
  })

  it('区域编辑的 Escape 返回焦点不会重新触发预览', async () => {
    sourceMode.value = 'region'
    state.value = { status: 'empty', document: { imageWidth: 100, imageHeight: 60, imagePngB64: 'cG5n', sourceLanguage: 'en', targetLanguage: 'zh-CN', blocks: [] } }
    const wrapper = createOverlay(true)
    const entry = wrapper.get('button[aria-label="编辑捕获区域"]')
    ;(entry.element as HTMLButtonElement).focus()
    await entry.trigger('click')
    expect(wrapper.find('.preview-menu').exists()).toBe(true)
    ;(wrapper.get('input[aria-label="测试坐标"]').element as HTMLInputElement).focus()
    await wrapper.trigger('keydown', { key: 'Escape' })
    expect(wrapper.find('.preview-menu').exists()).toBe(false)
    expect(document.activeElement).toBe(entry.element)
    wrapper.unmount()
    vi.useRealTimers()
  })

  it('翻译和设置直接在合并窗口的工具栏中操作', async () => {
    const wrapper = createOverlay()
    await wrapper.get('button[aria-label="翻译"]').trigger('click')
    expect(start).toHaveBeenCalledOnce()
    expect(sourceMode.value).toBe('region')
    await wrapper.get('button[aria-label="设置"]').trigger('click')
    expect(openSettings).toHaveBeenCalledOnce()
    state.value = { status: 'processing' }
    await wrapper.get('button[aria-label="翻译"]').trigger('click')
    expect(cancel).toHaveBeenCalledOnce()
    wrapper.unmount()
    vi.useRealTimers()
  })
})
