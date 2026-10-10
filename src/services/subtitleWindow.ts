import { LogicalSize } from '@tauri-apps/api/dpi'
import { getCurrentWindow } from '@tauri-apps/api/window'

export async function resizeSubtitle(width: number, height: number) {
  const window = getCurrentWindow()
  if (await window.isFullscreen() || await window.isMaximized()) return
  return window.setSize(new LogicalSize(width, height))
}

export function startSubtitleDrag() {
  return getCurrentWindow().startDragging()
}

export function startSubtitleResize(side: 'left' | 'right') {
  const direction = side === 'left' ? 'West' as const : 'East' as const
  return getCurrentWindow().startResizeDragging(direction)
}

export function getSubtitleAlwaysOnTop() {
  return getCurrentWindow().isAlwaysOnTop()
}

export function setSubtitleAlwaysOnTop(alwaysOnTop: boolean) {
  return getCurrentWindow().setAlwaysOnTop(alwaysOnTop)
}
