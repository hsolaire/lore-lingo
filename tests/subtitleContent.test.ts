import { mount } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import { ref } from 'vue'

vi.mock('@/composables/useTranslator', () => ({ useTranslator: () => ({ srcText: ref(''), tgtText: ref('') }) }))
import SubtitleContent from '@/components/subtitle/SubtitleContent.vue'

describe('字幕边角缩放', () => {
  it('保留两侧拖动命中区和正确方向，空状态清晰显示', async () => {
    const wrapper = mount(SubtitleContent)
    expect(wrapper.text()).toContain('译文会显示在这里')
    const left = wrapper.get('.resize-handle.left')
    const right = wrapper.get('.resize-handle.right')
    await left.trigger('mousedown', { button: 0 })
    await right.trigger('mousedown', { button: 0 })
    expect(wrapper.emitted('resize-mousedown')?.map(event => event[1])).toEqual(['left', 'right'])
    expect(left.attributes('title')).toBe('拖动调整窗口宽度')
    expect(wrapper.emitted('dragbar-mousedown')).toBeUndefined()
    wrapper.unmount()
  })
})
