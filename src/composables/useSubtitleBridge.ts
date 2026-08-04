import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { applyRemoteState, type TranslatorPayload } from './useTranslator'

export function useSubtitleBridge(onPeek: () => void) {
  const unlisten: UnlistenFn[] = []

  onMounted(async () => {
    // Register all event listeners in parallel so there is no window where
    // some events are wired and others are not (Fix I-4).
    const fns = await Promise.all([
      // Fix C-1: use applyRemoteState to update refs without triggering a
      // re-broadcast from the subtitle window's own watcher.
      listen<TranslatorPayload>('translator:update', ({ payload }) => {
        applyRemoteState(payload)
      }),
      listen('subtitle:show', () => { onPeek() }),
      listen('subtitle:peek', () => { onPeek() }),
      // Fix C-2: the subtitle window hides itself on hide/close requests.
      listen('subtitle:hide', async () => {
        try { await getCurrentWindow().hide() } catch (_) {}
      }),
      listen('subtitle:close', async () => {
        try { await getCurrentWindow().hide() } catch (_) {}
      }),
    ])
    unlisten.push(...fns)

  })

  onUnmounted(() => {
    unlisten.forEach(fn => fn())
  })
}
