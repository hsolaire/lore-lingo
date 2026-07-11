import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './useToast'

export type SourceMode = 'app' | 'region'

export interface WindowInfo {
  id: number
  title: string
  appName: string
  x: number
  y: number
  width: number
  height: number
}

// 单例捕获源状态
const sourceMode = ref<SourceMode>('app')
const selecting = ref(false)
const windows = ref<WindowInfo[]>([])
const selectedWindow = ref<WindowInfo | null>(null)
const loadingWindows = ref(false)
let selectTimer: ReturnType<typeof setTimeout> | undefined

export function useCaptureSource() {
  const { toast } = useToast()

  const targetName = computed(() => {
    if (sourceMode.value === 'app') {
      return selectedWindow.value?.title ?? '未选择窗口'
    }
    return '自定义区域'
  })

  const targetMeta = computed(() => {
    if (sourceMode.value === 'app') {
      if (!selectedWindow.value) return '点击下方选择目标窗口'
      const { appName, width, height } = selectedWindow.value
      return `${appName} · ${width}×${height}`
    }
    return '520 × 118 · 屏幕坐标 (700, 46)'
  })

  async function fetchWindows() {
    loadingWindows.value = true
    try {
      windows.value = await invoke<WindowInfo[]>('list_windows')
    } catch (e) {
      toast('无法获取窗口列表')
      windows.value = []
    } finally {
      loadingWindows.value = false
    }
  }

  function selectWindow(w: WindowInfo) {
    selectedWindow.value = w
    toast(`已选择：${w.title}`)
  }

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

  async function setSource(mode: SourceMode) {
    sourceMode.value = mode
    if (mode === 'app') {
      await fetchWindows()
    } else {
      startSelect()
    }
  }

  /** 重选：region 下重新框选，app 下重新获取窗口列表 */
  async function reselect() {
    if (sourceMode.value === 'region') {
      startSelect()
    } else {
      await fetchWindows()
      toast('窗口列表已刷新')
    }
  }

  return {
    sourceMode, selecting, loadingWindows,
    windows, selectedWindow,
    targetName, targetMeta,
    setSource, startSelect, reselect, selectWindow, fetchWindows,
  }
}
