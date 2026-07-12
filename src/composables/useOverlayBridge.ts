import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTranslator } from './useTranslator'

export type TranslatorPayload = {
  srcText: string
  tgtText: string
  langA: string
  langB: string
  pinned: boolean
}

export function useOverlayBridge(onPeek: () => void) {
  const { srcText, tgtText, langA, langB, pinned } = useTranslator()
  const unlisten: UnlistenFn[] = []

  onMounted(async () => {
    unlisten.push(
      await listen<TranslatorPayload>('translator:update', ({ payload }) => {
        srcText.value  = payload.srcText
        tgtText.value  = payload.tgtText
        langA.value    = payload.langA
        langB.value    = payload.langB
        pinned.value   = payload.pinned
      }),
      await listen('lyric:enter', () => {
        onPeek()
      }),
      await listen('lyric:peek', () => {
        onPeek()
      }),
    )

    // Keep overlay always-on-top when shown
    try {
      await getCurrentWindow().setAlwaysOnTop(true)
    } catch (_) {}
  })

  onUnmounted(() => {
    unlisten.forEach(fn => fn())
  })
}
