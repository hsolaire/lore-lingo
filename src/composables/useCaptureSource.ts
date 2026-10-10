import { computed, ref } from 'vue'
import { desktopClient } from '@/services/desktopClient'
import type { CapturePermissionStatus, CaptureSourceSummary } from '@/types/desktop'
import { useToast } from './useToast'
import { useCaptureTranslation } from './useCaptureTranslation'

export type SourceMode = 'app' | 'region'

const sourceMode = ref<SourceMode>('app')
const windowList = ref<CaptureSourceSummary[]>([])
const selectedWindow = ref<CaptureSourceSummary | null>(null)
const listOpen = ref(false)
const listLoading = ref(false)

export function useCaptureSource() {
  const { toast } = useToast()
  const captureTranslation = useCaptureTranslation()
  const selecting = computed(() => captureTranslation.state.value.status === 'selecting')

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
    const status = captureTranslation.state.value.status
    if (status === 'selecting') return '请在屏幕上拖拽选择翻译区域'
    if (status === 'processing') return '正在识别文字并翻译'
    if (status === 'success') return '区域翻译已完成'
    if (status === 'empty') return '所选区域未识别到文字'
    return '点击开始框选屏幕区域'
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

  async function startSelect() {
    await captureTranslation.start()
    if (captureTranslation.state.value.status === 'error') {
      toast(captureTranslation.state.value.error.message)
    }
  }

  function setSource(mode: SourceMode) {
    sourceMode.value = mode
    if (mode === 'region') {
      listOpen.value = false
      void startSelect()
    }
  }

  function reselect() {
    if (sourceMode.value === 'region') void startSelect()
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
