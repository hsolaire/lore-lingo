import { beforeEach, describe, expect, it, vi } from 'vitest'

const { showOrFocus, hide } = vi.hoisted(() => ({
  showOrFocus: vi.fn<() => Promise<void>>(),
  hide: vi.fn<() => Promise<void>>(),
}))

vi.mock('@/services/windowClient', () => ({
  windowClient: {
    showOrFocus,
    hide,
  },
}))

import { useWindowNavigation } from '@/composables/useWindowNavigation'

describe('窗口导航', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    showOrFocus.mockResolvedValue(undefined)
    hide.mockResolvedValue(undefined)
  })

  it('使用稳定窗口标签打开设置页和主面板', async () => {
    const navigation = useWindowNavigation('main')

    await navigation.openSettings()
    await navigation.openMain()

    expect(showOrFocus).toHaveBeenNthCalledWith(1, 'settings')
    expect(showOrFocus).toHaveBeenNthCalledWith(2, 'main')
  })

  it('隐藏组合式函数所属的当前窗口', async () => {
    const navigation = useWindowNavigation('settings')

    await navigation.hideCurrent()

    expect(hide).toHaveBeenCalledWith('settings')
  })
})
