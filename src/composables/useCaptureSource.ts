import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './useToast'

export type SourceMode = 'app' | 'region'

export interface WindowInfo {
  id: number
  title: string
  exe: string
  icon_b64: string | null
}

const sourceMode = ref<SourceMode>('app')
const selecting = ref(false)
const windowList = ref<WindowInfo[]>([])
const selectedWindow = ref<WindowInfo | null>(null)
const listOpen = ref(false)
const listLoading = ref(false)
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
      return selectedWindow.value?.exe ?? '点击下方选择目标窗口'
    }
    return '520 × 118 · 屏幕坐标 (700, 46)'
  })

  async function fetchWindows() {
    listLoading.value = true
    try {
      windowList.value = await invoke<WindowInfo[]>('list_windows')
    } catch {
      windowList.value = []
    } finally {
      listLoading.value = false
    }
  }

  function selectWindow(w: WindowInfo) {
    selectedWindow.value = w
    listOpen.value = false
  }

  function toggleList() {
    if (!listOpen.value) {
      listOpen.value = true
      fetchWindows()
    } else {
      listOpen.value = false
    }
  }

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
    if (mode === 'region') {
      listOpen.value = false
      startSelect()
    }
  }

  function reselect() {
    if (sourceMode.value === 'region') startSelect()
    else toggleList()
  }

  return {
    sourceMode,
    selecting,
    windowList,
    selectedWindow,
    listOpen,
    listLoading,
    targetName,
    targetMeta,
    fetchWindows,
    selectWindow,
    toggleList,
    setSource,
    reselect,
  }
}
