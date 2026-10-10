import { mount } from '@vue/test-utils'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { defineComponent, h, ref } from 'vue'
import { useOverlayToolbar } from '@/composables/useOverlayToolbar'

function createToolbar() {
  const keepOpen = ref(false)
  let toolbar!: ReturnType<typeof useOverlayToolbar>
  const wrapper = mount(defineComponent({
    setup() { toolbar = useOverlayToolbar(keepOpen); return () => h('main') },
  }))
  return { toolbar, wrapper, keepOpen }
}

describe('工具栏自动收起与固定', () => {
  beforeEach(() => { vi.useFakeTimers(); localStorage.clear() })
  afterEach(() => { vi.useRealTimers() })

  it('移开后延迟收起，快速移回会取消收起', async () => {
    const { toolbar, wrapper } = createToolbar()
    toolbar.reveal()
    toolbar.leave()
    await vi.advanceTimersByTimeAsync(300)
    expect(toolbar.visible.value).toBe(true)
    toolbar.reveal()
    await vi.advanceTimersByTimeAsync(1000)
    expect(toolbar.visible.value).toBe(true)
    toolbar.leave()
    await vi.advanceTimersByTimeAsync(450)
    expect(toolbar.visible.value).toBe(false)
    wrapper.unmount()
  })

  it('固定后移开仍显示，并在再次挂载时恢复偏好', async () => {
    const first = createToolbar()
    first.toolbar.toggleToolbarPinned()
    first.toolbar.leave()
    await vi.advanceTimersByTimeAsync(3000)
    expect(first.toolbar.visible.value).toBe(true)
    first.wrapper.unmount()
    const second = createToolbar()
    expect(second.toolbar.toolbarPinned.value).toBe(true)
    second.wrapper.unmount()
  })

  it('键盘焦点保持显示，移出窗口后恢复自动收起', async () => {
    const { toolbar, wrapper } = createToolbar()
    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab' }))
    toolbar.onFocusIn()
    toolbar.leave()
    await vi.advanceTimersByTimeAsync(3000)
    expect(toolbar.visible.value).toBe(true)
    toolbar.onFocusOut(new FocusEvent('focusout', { relatedTarget: null }))
    await vi.advanceTimersByTimeAsync(450)
    expect(toolbar.visible.value).toBe(false)
    wrapper.unmount()
  })

  it('关闭已展开面板后恢复自动收起，销毁时清理计时器', async () => {
    const { toolbar, wrapper, keepOpen } = createToolbar()
    keepOpen.value = true
    toolbar.leave()
    await vi.advanceTimersByTimeAsync(3000)
    expect(toolbar.visible.value).toBe(true)
    keepOpen.value = false
    await vi.advanceTimersByTimeAsync(450)
    expect(toolbar.visible.value).toBe(false)
    toolbar.leave()
    wrapper.unmount()
    expect(vi.getTimerCount()).toBe(0)
  })
})
