import { getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'

import type { WindowSummary } from '@/types/desktop'
import { invokeCommand } from './tauriClient'

type CurrentWindow = ReturnType<typeof getCurrentWindow>

function runWithCurrentWindow(operation: (window: CurrentWindow) => Promise<void>): Promise<void> {
  try {
    return operation(getCurrentWindow())
  } catch (_) {
    return Promise.resolve()
  }
}

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
  setMainControlsVisible(visible: boolean): Promise<void> {
    return invokeCommand<void>('set_main_window_controls_visible', { visible })
  },
}

export const currentWindowClient = {
  close(): Promise<void> {
    return runWithCurrentWindow((window) => window.close())
  },
  hide(): Promise<void> {
    return runWithCurrentWindow((window) => window.hide())
  },
  minimize(): Promise<void> {
    return runWithCurrentWindow((window) => window.minimize())
  },
  setAlwaysOnTop(alwaysOnTop: boolean): Promise<void> {
    return runWithCurrentWindow((window) => window.setAlwaysOnTop(alwaysOnTop))
  },
  toggleMaximize(): Promise<void> {
    return runWithCurrentWindow((window) => window.toggleMaximize())
  },
  startDragging(): Promise<void> {
    return runWithCurrentWindow((window) => window.startDragging())
  },
  setSize(width: number, height: number): Promise<void> {
    return runWithCurrentWindow((window) => window.setSize(new LogicalSize(width, height)))
  },
}
