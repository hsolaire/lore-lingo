import { ref } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { emit } from '@tauri-apps/api/event'
import { useToast } from './useToast'
import { useSettings } from './useSettings'

// peekSignal is only used within the overlay window now, but keep it
// so LyricBar (which also runs in overlay) can still import this composable.
const peekSignal = ref(0)

async function getOverlay(): Promise<WebviewWindow | null> {
  try {
    return WebviewWindow.getByLabel('overlay')
  } catch (_) {
    return null
  }
}

export function useLyricMode() {
  const { toast } = useToast()
  const { closeDrawer } = useSettings()

  async function enter() {
    closeDrawer()
    const overlay = await getOverlay()
    if (overlay) {
      await overlay.show()
      await overlay.setFocus()
    }
    await emit('lyric:enter', {})
    toast('已切换为屏幕歌词条 · 置顶显示')
  }

  async function exit() {
    const overlay = await getOverlay()
    if (overlay) await overlay.hide()
    await emit('lyric:exit', {})
    toast('已回到面板视图')
  }

  async function close() {
    const overlay = await getOverlay()
    if (overlay) await overlay.hide()
    await emit('lyric:close', {})
    toast('歌词条已关闭')
  }

  // lyricMode ref is no longer used in main window — kept for API compatibility
  // with any component that imports it (they can safely ignore it)
  const lyricMode = ref(false)

  return { lyricMode, peekSignal, enter, exit, close }
}
