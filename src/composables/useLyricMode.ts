import { ref } from 'vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { useToast } from './useToast'
import { useSettings } from './useSettings'

const lyricMode = ref(false)
const peekSignal = ref(0)

// 主面板窗口尺寸
const PANEL_W = 420
const PANEL_H = 640
// 歌词条窗口尺寸（足够容纳歌词条 + 控制排，其余透明）
const LYRIC_W = 820
const LYRIC_H = 200

async function resizeWindow(w: number, h: number) {
  try {
    await getCurrentWindow().setSize(new LogicalSize(w, h))
  } catch (_) {}
}

export function useLyricMode() {
  const { toast } = useToast()
  const { closeDrawer } = useSettings()

  async function enter() {
    lyricMode.value = true
    closeDrawer()
    peekSignal.value++
    await resizeWindow(LYRIC_W, LYRIC_H)
    toast('已切换为屏幕歌词条 · 置顶显示')
  }

  async function exit() {
    lyricMode.value = false
    await resizeWindow(PANEL_W, PANEL_H)
    toast('已回到面板视图')
  }

  async function close() {
    lyricMode.value = false
    await resizeWindow(PANEL_W, PANEL_H)
    toast('歌词条已关闭')
  }

  return { lyricMode, peekSignal, enter, exit, close }
}
