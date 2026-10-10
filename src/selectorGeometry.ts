import type { LogicalRect } from '@/types/capturedLayout'

export type SelectionPoint = { x: number; y: number }

export function createLogicalRect(
  start: SelectionPoint | null,
  current: SelectionPoint | null,
): LogicalRect | null {
  if (!start || !current) return null
  return {
    x: Math.min(start.x, current.x),
    y: Math.min(start.y, current.y),
    width: Math.abs(current.x - start.x),
    height: Math.abs(current.y - start.y),
  }
}
