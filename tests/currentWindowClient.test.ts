import { beforeEach, describe, expect, it, vi } from 'vitest'

const { close, hide, minimize, setAlwaysOnTop, toggleMaximize, startDragging, getCurrentWindow } = vi.hoisted(() => ({
  close: vi.fn<() => Promise<void>>(),
  hide: vi.fn<() => Promise<void>>(),
  minimize: vi.fn<() => Promise<void>>(),
  setAlwaysOnTop: vi.fn<(alwaysOnTop: boolean) => Promise<void>>(),
  toggleMaximize: vi.fn<() => Promise<void>>(),
  startDragging: vi.fn<() => Promise<void>>(),
  getCurrentWindow: vi.fn(),
}))

vi.mock('@tauri-apps/api/window', () => ({ getCurrentWindow }))

import { currentWindowClient } from '@/services/windowClient'

describe('当前窗口客户端', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    close.mockResolvedValue(undefined)
    hide.mockResolvedValue(undefined)
    minimize.mockResolvedValue(undefined)
    setAlwaysOnTop.mockResolvedValue(undefined)
    toggleMaximize.mockResolvedValue(undefined)
    startDragging.mockResolvedValue(undefined)
    getCurrentWindow.mockReturnValue({
      close,
      hide,
      minimize,
      setAlwaysOnTop,
      toggleMaximize,
      startDragging,
    })
  })

  it('统一转发当前窗口控制操作', async () => {
    await currentWindowClient.close()
    await currentWindowClient.hide()
    await currentWindowClient.minimize()
    await currentWindowClient.setAlwaysOnTop(true)
    await currentWindowClient.toggleMaximize()
    await currentWindowClient.startDragging()

    expect(close).toHaveBeenCalledOnce()
    expect(hide).toHaveBeenCalledOnce()
    expect(minimize).toHaveBeenCalledOnce()
    expect(setAlwaysOnTop).toHaveBeenCalledWith(true)
    expect(toggleMaximize).toHaveBeenCalledOnce()
    expect(startDragging).toHaveBeenCalledOnce()
  })

  it('浏览器预览无法取得 Tauri 窗口时安全降级', async () => {
    getCurrentWindow.mockImplementation(() => {
      throw new Error('当前不在 Tauri 上下文')
    })

    await expect(currentWindowClient.close()).resolves.toBeUndefined()
    await expect(currentWindowClient.startDragging()).resolves.toBeUndefined()
  })
})
