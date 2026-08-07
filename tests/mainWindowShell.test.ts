import { mount, shallowMount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { ref } from 'vue'

const { openSettings, openSubtitle, close, minimize, toggleMaximize, startDragging } = vi.hoisted(
  () => ({
    openSettings: vi.fn<() => Promise<void>>(),
    openSubtitle: vi.fn<() => Promise<void>>(),
    close: vi.fn<() => Promise<void>>(),
    minimize: vi.fn<() => Promise<void>>(),
    toggleMaximize: vi.fn<() => Promise<void>>(),
    startDragging: vi.fn<() => Promise<void>>(),
  }),
)

vi.mock('@/composables/useWindowNavigation', () => ({
  useWindowNavigation: () => ({ openSettings, openSubtitle }),
}))

vi.mock('@/services/windowClient', () => ({
  currentWindowClient: { close, minimize, toggleMaximize, startDragging },
}))

vi.mock('@/composables/useTranslator', () => ({
  useTranslator: () => ({ pinned: ref(false), togglePin: vi.fn() }),
}))

import MainPanel from '@/components/main-panel/MainPanel.vue'
import TitleBar from '@/components/main-panel/TitleBar.vue'

describe('主页窗口壳', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    openSettings.mockResolvedValue(undefined)
    openSubtitle.mockResolvedValue(undefined)
    close.mockResolvedValue(undefined)
    minimize.mockResolvedValue(undefined)
    toggleMaximize.mockResolvedValue(undefined)
    startDragging.mockResolvedValue(undefined)
  })

  it('设置按钮打开独立设置窗口', async () => {
    const wrapper = mount(TitleBar, {
      global: { stubs: { Icon: true } },
    })

    await wrapper.get('button[aria-label="设置"]').trigger('click')

    expect(openSettings).toHaveBeenCalledOnce()
  })

  it('悬浮字幕按钮通过统一窗口导航打开', async () => {
    const wrapper = mount(TitleBar, {
      global: { stubs: { Icon: true } },
    })

    await wrapper.get('button[aria-label="打开悬浮字幕"]').trigger('click')

    expect(openSubtitle).toHaveBeenCalledOnce()
  })

  it('主页不再挂载内嵌设置抽屉', () => {
    const wrapper = shallowMount(MainPanel, {
      global: {
        stubs: {
          SettingsDrawer: { template: '<aside data-testid="settings-drawer" />' },
        },
      },
    })

    expect(wrapper.find('[data-testid="settings-drawer"]').exists()).toBe(false)
  })
})
