import type { ShortcutBindingSummary } from '@/types/settings'
import { invokeCommand } from './tauriClient'

export const shortcutClient = {
  list(): Promise<ShortcutBindingSummary[]> {
    return invokeCommand<ShortcutBindingSummary[]>('list_shortcut_bindings')
  },
}
