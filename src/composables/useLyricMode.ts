import { ref } from 'vue'
import { useToast } from './useToast'
import { useSettings } from './useSettings'

// 单例形态标志：主面板 ↔ 屏幕歌词条。
const lyricMode = ref(false)
/** 进入歌词条时先亮一下控制排，再淡出——由 LyricBar 监听触发 peek */
const peekSignal = ref(0)

export function useLyricMode() {
  const { toast } = useToast()
  const { closeDrawer } = useSettings()

  function enter() {
    lyricMode.value = true
    closeDrawer()
    peekSignal.value++ // 触发 LyricBar 的 peek()
    toast('已切换为屏幕歌词条 · 置顶显示')
  }

  function exit() {
    lyricMode.value = false
    toast('已回到面板视图')
  }

  function close() {
    lyricMode.value = false
    toast('歌词条已关闭')
  }

  return { lyricMode, peekSignal, enter, exit, close }
}
