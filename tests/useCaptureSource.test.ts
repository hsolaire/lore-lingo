import { beforeEach, describe, expect, it, vi } from 'vitest'

const { capturePermission, requestCapturePermission, sources, toast } = vi.hoisted(() => ({
  capturePermission: vi.fn(),
  requestCapturePermission: vi.fn(),
  sources: vi.fn(),
  toast: vi.fn(),
}))

vi.mock('@/services/desktopClient', () => ({
  desktopClient: {
    capturePermission,
    requestCapturePermission,
    sources,
  },
}))

vi.mock('@/composables/useToast', () => ({
  useToast: () => ({ toast }),
}))

import { useCaptureSource } from '@/composables/useCaptureSource'

const windowSource = {
  sourceId: 'capture-source-1-1',
  catalogRevision: 1,
  kind: 'window' as const,
  title: '资料窗口',
  applicationName: '示例应用',
  iconDataUrl: null,
  monitor: null,
  capturable: true,
}

const screenSource = {
  ...windowSource,
  sourceId: 'capture-source-1-2',
  kind: 'screen' as const,
  title: '主显示器',
}

const blockedWindow = {
  ...windowSource,
  sourceId: 'capture-source-1-3',
  title: '受保护窗口',
  capturable: false,
}

describe('捕获来源组合式函数', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    capturePermission.mockResolvedValue({
      status: 'not_required',
      canRequest: false,
      action: null,
    })
    requestCapturePermission.mockResolvedValue({
      status: 'granted',
      canRequest: false,
      action: null,
    })
    sources.mockResolvedValue({
      catalogRevision: 1,
      sources: [windowSource, screenSource, blockedWindow],
    })
  })

  it('通过统一桌面客户端加载并筛选可捕获窗口', async () => {
    const captureSource = useCaptureSource()

    await captureSource.fetchWindows()

    expect(capturePermission).toHaveBeenCalledOnce()
    expect(sources).toHaveBeenCalledOnce()
    expect(captureSource.windowList.value).toEqual([windowSource])
  })

  it('仅在用户打开来源列表后请求尚未决定的 macOS 权限', async () => {
    capturePermission.mockResolvedValue({
      status: 'not_determined',
      canRequest: true,
      action: null,
    })
    const captureSource = useCaptureSource()

    await captureSource.fetchWindows()

    expect(requestCapturePermission).toHaveBeenCalledOnce()
    expect(sources).toHaveBeenCalledOnce()
  })

  it('权限被拒绝时不读取来源目录并清空窗口列表', async () => {
    capturePermission.mockResolvedValue({
      status: 'denied',
      canRequest: false,
      action: 'open_system_settings',
    })
    const captureSource = useCaptureSource()

    await captureSource.fetchWindows()

    expect(sources).not.toHaveBeenCalled()
    expect(captureSource.windowList.value).toEqual([])
    expect(toast).toHaveBeenCalledWith('未获得屏幕录制权限，请在系统设置中授权')
  })

  it('来源目录刷新后清除已过期的窗口选择', async () => {
    const captureSource = useCaptureSource()
    await captureSource.fetchWindows()
    captureSource.selectWindow(windowSource)
    sources.mockResolvedValue({
      catalogRevision: 2,
      sources: [{
        ...windowSource,
        sourceId: 'capture-source-2-1',
        catalogRevision: 2,
      }],
    })

    await captureSource.fetchWindows()

    expect(captureSource.selectedWindow.value).toBeNull()
  })
})
