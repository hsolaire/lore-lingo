import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { emit } from '@tauri-apps/api/event'
import { useToast } from './useToast'
import { useSettings } from './useSettings'
import { useCaptureSource } from './useCaptureSource'

async function getSubtitleWindow(): Promise<WebviewWindow | null> {
  try {
    return WebviewWindow.getByLabel('subtitle')
  } catch (_) {
    return null
  }
}

export function useSubtitle() {
  const { toast } = useToast()
  const { closeDrawer } = useSettings()
  const { listOpen } = useCaptureSource()

  async function show() {
    closeDrawer()
    listOpen.value = false
    const subtitleWindow = await getSubtitleWindow()
    if (subtitleWindow) {
      await subtitleWindow.show()
    }
    await emit('subtitle:show', {})
    toast('已打开悬浮字幕')
  }

  async function hide() {
    const subtitleWindow = await getSubtitleWindow()
    if (subtitleWindow) await subtitleWindow.hide()
    await emit('subtitle:hide', {})
    toast('已回到面板视图')
  }

  async function close() {
    const subtitleWindow = await getSubtitleWindow()
    if (subtitleWindow) await subtitleWindow.hide()
    await emit('subtitle:close', {})
    toast('悬浮字幕已关闭')
  }

  return { show, hide, close }
}
