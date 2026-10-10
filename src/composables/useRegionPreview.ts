import { onMounted, onUnmounted, readonly, ref, shallowRef } from 'vue'
import { captureTranslationClient } from '@/services/captureTranslationClient'
import type { CapturedRegion, RegionBounds, RegionPreview } from '@/types/capturedLayout'

export function useRegionPreview() {
  const region = shallowRef<CapturedRegion | null>(null)
  const frame = shallowRef<RegionPreview | null>(null)
  const error = ref('')
  const saving = ref(false)
  const loading = ref(true)
  let generation = 0
  let closed = false
  let paused = false
  let timer: ReturnType<typeof setTimeout> | undefined
  let inFlight: Promise<void> | null = null

  function message(reason: unknown) {
    return typeof reason === 'object' && reason && 'message' in reason ? String(reason.message) : '区域预览不可用，请重试。'
  }

  async function refresh() {
    if (closed || paused || inFlight || !region.value) return
    const token = generation
    const revision = region.value.revision
    clearTimeout(timer)
    // 步骤 1. 一次只请求一帧；关闭面板或改变坐标后丢弃旧响应。
    const request = (async () => {
      try {
        const next = await captureTranslationClient.regionPreview(revision)
        if (closed || paused || generation !== token || next.revision !== region.value?.revision) return
        frame.value = next
        error.value = ''
        // 步骤 2. 本帧完成后再安排下一帧，慢捕获不会产生排队。
        timer = setTimeout(() => { void refresh() }, 500)
      } catch (reason: unknown) {
        if (!closed && generation === token) error.value = message(reason)
      } finally {
        if (!closed && generation === token) loading.value = false
      }
    })()
    inFlight = request
    await request
    if (inFlight === request) inFlight = null
  }

  async function stop() {
    paused = true
    generation++
    clearTimeout(timer)
    await inFlight
  }

  async function apply(bounds: RegionBounds) {
    if (!region.value || saving.value) return
    saving.value = true
    await stop()
    try {
      // 步骤 3. 等待旧帧退出后原子更新坐标，再获取新区域的第一帧。
      await inFlight
      if (closed) return
      const next = await captureTranslationClient.updateRegion(region.value.revision, bounds)
      if (closed) return
      region.value = next
      frame.value = null
      loading.value = true
      paused = false
      await refresh()
    } catch (reason: unknown) {
      if (!closed) error.value = message(reason)
    } finally { if (!closed) { saving.value = false; paused = false } }
  }

  onMounted(async () => {
    try {
      const next = await captureTranslationClient.region()
      if (closed) return
      region.value = next
      if (!next) { error.value = '请先框选区域。'; loading.value = false; return }
      await refresh()
    } catch (reason: unknown) { if (!closed) { error.value = message(reason); loading.value = false } }
  })
  onUnmounted(() => { closed = true; generation++; clearTimeout(timer) })
  return { region: readonly(region), frame: readonly(frame), error: readonly(error), loading: readonly(loading), saving: readonly(saving), refresh, apply, stop }
}
