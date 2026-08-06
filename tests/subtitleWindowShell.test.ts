import { mount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { ref } from 'vue'

const { openMain, hideCurrent } = vi.hoisted(() => ({
  openMain: vi.fn<() => Promise<void>>(),
  hideCurrent: vi.fn<() => Promise<void>>(),
}))

vi.mock('@/composables/useWindowNavigation', () => ({
  useWindowNavigation: () => ({ openMain, hideCurrent }),
}))

vi.mock('@/composables/useTranslator', () => ({
  useTranslator: () => ({ sessionActive: ref(false) }),
}))

import SubtitleControls from '@/components/subtitle/SubtitleControls.vue'

describe('悬浮字幕窗口导航', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    openMain.mockResolvedValue(undefined)
    hideCurrent.mockResolvedValue(undefined)
  })

  it('回到面板时先聚焦主页再隐藏悬浮字幕', async () => {
    const wrapper = mount(SubtitleControls, {
      props: { pinned: true },
      global: { stubs: { Icon: true } },
    })

    await wrapper.get('button[aria-label="回到面板视图"]').trigger('click')

    expect(openMain).toHaveBeenCalledOnce()
    expect(hideCurrent).toHaveBeenCalledOnce()
    expect(openMain.mock.invocationCallOrder[0]).toBeLessThan(hideCurrent.mock.invocationCallOrder[0])
  })

  it('关闭悬浮字幕时隐藏当前窗口', async () => {
    const wrapper = mount(SubtitleControls, {
      props: { pinned: false },
      global: { stubs: { Icon: true } },
    })

    await wrapper.get('button[aria-label="关闭悬浮字幕"]').trigger('click')

    expect(hideCurrent).toHaveBeenCalledOnce()
  })
})
