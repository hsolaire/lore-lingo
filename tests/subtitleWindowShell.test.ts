import { mount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const { startDragging, minimize, close } = vi.hoisted(() => ({
  startDragging: vi.fn<() => Promise<void>>(), minimize: vi.fn<() => Promise<void>>(), close: vi.fn<() => Promise<void>>(),
}))
vi.mock('@/services/windowClient', () => ({ currentWindowClient: { startDragging, minimize, close } }))

import SubtitleControls from '@/components/subtitle/SubtitleControls.vue'

describe('合并悬浮字幕工具栏', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    startDragging.mockResolvedValue(undefined)
  })

  it('独立提供固定工具栏和窗口置顶，不再显示回到面板按钮', async () => {
    const wrapper = mount(SubtitleControls, { props: { platform: 'macos', toolbarPinned: false, alwaysOnTop: true } })
    expect(wrapper.find('button[aria-label="回到面板视图"]').exists()).toBe(false)
    expect(wrapper.get('button[aria-label="固定工具栏"]').attributes('aria-pressed')).toBe('false')
    expect(wrapper.get('button[aria-label="窗口置顶"]').attributes('aria-pressed')).toBe('true')
    await wrapper.get('button[aria-label="固定工具栏"]').trigger('click')
    await wrapper.get('button[aria-label="窗口置顶"]').trigger('click')
    expect(wrapper.emitted('toggle-toolbar-pinned')).toHaveLength(1)
    expect(wrapper.emitted('toggle-always-on-top')).toHaveLength(1)
    wrapper.unmount()
  })

  it('翻译期间按钮变为取消，并且点击控件不会拖动窗口', async () => {
    const wrapper = mount(SubtitleControls, { props: { platform: 'macos', busy: true } })
    const cancel = wrapper.get('button[aria-label="取消捕获"]')
    await cancel.trigger('mousedown', { button: 0 })
    await cancel.trigger('click')
    expect(startDragging).not.toHaveBeenCalled()
    expect(wrapper.emitted('translate')).toHaveLength(1)
    expect(wrapper.find('button[aria-label="关闭窗口"]').exists()).toBe(false)
    wrapper.unmount()
  })

  it('只显示四个左侧入口，捕获成功后区域按钮提供缩略图预览事件', async () => {
    const wrapper = mount(SubtitleControls, { props: { platform: 'macos', captured: true } })
    expect(wrapper.find('.action-group')?.findAll('button')).toHaveLength(4)
    expect(wrapper.get('button[aria-label="翻译选项"]').attributes('aria-haspopup')).toBe('dialog')
    expect(wrapper.get('button[aria-label="捕获窗口"]').attributes('aria-haspopup')).toBe('dialog')
    const region = wrapper.get('button[aria-label="编辑捕获区域"]')
    expect(region.text()).toContain('已捕获')
    await region.trigger('mouseenter')
    await region.trigger('mouseleave')
    expect(wrapper.emitted('preview')).toEqual([[true], [false]])
    expect(wrapper.find('button[aria-label="配置截图翻译"]').exists()).toBe(false)
    wrapper.unmount()
  })
})
