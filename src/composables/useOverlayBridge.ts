import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { applyRemoteState, type TranslatorPayload } from './useTranslator'

export function useOverlayBridge(onPeek: () => void) {
  const unlisten: UnlistenFn[] = []

  onMounted(async () => {
    // Register all event listeners in parallel so there is no window where
    // some events are wired and others are not (Fix I-4).
    const fns = await Promise.all([
      // Fix C-1: use applyRemoteState to update refs without triggering a
      // re-broadcast from the overlay's own watcher.
      listen<TranslatorPayload>('translator:update', ({ payload }) => {
        applyRemoteState(payload)
      }),
      listen('lyric:enter', () => { onPeek() }),
      listen('lyric:peek',  () => { onPeek() }),
      // Fix C-2: overlay hides itself when the main window requests exit/close.
      listen('lyric:exit', async () => {
        try { await getCurrentWindow().hide() } catch (_) {}
      }),
      listen('lyric:close', async () => {
        try { await getCurrentWindow().hide() } catch (_) {}
      }),
    ])
    unlisten.push(...fns)

    // Keep overlay always-on-top when shown
    try {
      await getCurrentWindow().setAlwaysOnTop(true)
    } catch (_) {}
  })

  onUnmounted(() => {
    unlisten.forEach(fn => fn())
  })
}
