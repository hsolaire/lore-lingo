import { computed, ref } from 'vue'
import { useToast } from './useToast'

export type SourceMode = 'app' | 'region'

// 单例捕获源状态：app / region 切换，目标 name/meta 由模式派生，框选进行中标志。
const sourceMode = ref<SourceMode>('app')
const selecting = ref(false)
let selectTimer: ReturnType<typeof setTimeout> | undefined

export function useCaptureSource() {
  const { toast } = useToast()

  const targetName = computed(() =>
    sourceMode.value === 'app' ? 'VLC · 媒体播放器' : '自定义区域',
  )
  const targetMeta = computed(() =>
    sourceMode.value === 'app'
      ? '窗口 · 1920×1080 · 采样 2fps'
      : '520 × 118 · 屏幕坐标 (700, 46)',
  )

  /** 模拟框选：亮起提示框约 1.9s 后收起 */
  function startSelect() {
    selecting.value = true
    toast('拖拽屏幕以框选区域…')
    clearTimeout(selectTimer)
    selectTimer = setTimeout(() => {
      selecting.value = false
      toast('区域已捕获')
    }, 1900)
  }

  function setSource(mode: SourceMode) {
    sourceMode.value = mode
    if (mode === 'region') startSelect()
  }

  /** 重选：region 下重新框选，app 下提示重新捕获窗口 */
  function reselect() {
    if (sourceMode.value === 'region') startSelect()
    else toast('已重新捕获窗口')
  }

  return { sourceMode, selecting, targetName, targetMeta, setSource, startSelect, reselect }
}
