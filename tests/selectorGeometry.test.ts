import { describe, expect, it } from 'vitest'

import { createLogicalRect } from '@/selectorGeometry'

describe('选择器逻辑矩形', () => {
  it('支持从任意方向拖拽并保持左上角原点', () => {
    expect(createLogicalRect({ x: 80, y: 60 }, { x: 20, y: 10 })).toEqual({
      x: 20, y: 10, width: 60, height: 50,
    })
  })

  it('缺少起点或终点时不生成矩形', () => {
    expect(createLogicalRect(null, { x: 20, y: 10 })).toBeNull()
  })
})
