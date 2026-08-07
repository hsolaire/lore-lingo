import { computed, ref } from 'vue'
import { desktopClient } from '@/services/desktopClient'
import type { CapturePermissionStatus, CaptureSourceSummary } from '@/types/desktop'
import { useToast } from './useToast'

export type SourceMode = 'app' | 'region'

const sourceMode = ref<SourceMode>('app')
const selecting = ref(false)
const windowList = ref<CaptureSourceSummary[]>([])
const selectedWindow = ref<CaptureSourceSummary | null>(null)
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
      return selectedWindow.value?.applicationName ?? '点击下方选择目标窗口'
    }
    return '520 × 118 · 屏幕坐标 (700, 46)'
  })

  async function fetchWindows() {
    listLoading.value = true
    try {
      let permission = await desktopClient.capturePermission()
      if (permission.status === 'not_determined' && permission.canRequest) {
        permission = await desktopClient.requestCapturePermission()
      }

      const allowedStatuses: CapturePermissionStatus[] = ['granted', 'not_required']
      if (!allowedStatuses.includes(permission.status)) {
        windowList.value = []
        selectedWindow.value = null
        toast('未获得屏幕录制权限，请在系统设置中授权')
        return
      }

      const catalog = await desktopClient.sources()
      const windows = catalog.sources.filter(
        source => source.kind === 'window' && source.capturable,
      )
      windowList.value = windows

      if (
        selectedWindow.value
        && !windows.some(source => source.sourceId === selectedWindow.value?.sourceId)
      ) {
        selectedWindow.value = null
      }
    } catch {
      windowList.value = []
      selectedWindow.value = null
      toast('加载窗口失败，请稍后重试')
    } finally {
      listLoading.value = false
    }
  }

  function selectWindow(w: CaptureSourceSummary) {
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
