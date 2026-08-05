import type { WindowSummary } from '@/types/desktop'
import { invokeCommand } from './tauriClient'

export const windowClient = {
  list(): Promise<WindowSummary[]> {
    return invokeCommand<WindowSummary[]>('list_app_windows')
  },
  showOrFocus(label: string): Promise<void> {
    return invokeCommand<void>('show_or_focus_window', { label })
  },
  hide(label: string): Promise<void> {
    return invokeCommand<void>('hide_window', { label })
  },
}
