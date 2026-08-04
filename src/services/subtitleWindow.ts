import { LogicalSize } from '@tauri-apps/api/dpi'
import { getCurrentWindow } from '@tauri-apps/api/window'

const subtitleWindow = getCurrentWindow()

export function resizeSubtitle(width: number, height: number) {
  return subtitleWindow.setSize(new LogicalSize(width, height))
}

export function startSubtitleDrag() {
  return subtitleWindow.startDragging()
}

export function startSubtitleResize(side: 'left' | 'right') {
  const direction = side === 'left' ? 'West' as const : 'East' as const
  return subtitleWindow.startResizeDragging(direction)
}

export function getSubtitleAlwaysOnTop() {
  return subtitleWindow.isAlwaysOnTop()
}

export function setSubtitleAlwaysOnTop(alwaysOnTop: boolean) {
  return subtitleWindow.setAlwaysOnTop(alwaysOnTop)
}
