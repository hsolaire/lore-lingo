import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { enableAutoUnmount, mount } from '@vue/test-utils'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

const openMain = vi.fn<() => Promise<void>>().mockResolvedValue(undefined)
const hideCurrent = vi.fn<() => Promise<void>>().mockResolvedValue(undefined)

vi.mock('@/composables/useWindowNavigation', () => ({
  useWindowNavigation: () => ({ openMain, hideCurrent }),
}))

import SettingsShell from '@/components/settings/SettingsShell.vue'

enableAutoUnmount(afterEach)

describe('独立设置页', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('提供翻译、快捷键、OCR、字幕库四个一级导航', () => {
    const wrapper = mount(SettingsShell)
    const tabs = wrapper.findAll('[role="tab"]')

    expect(tabs.map((tab) => tab.text())).toEqual(['翻译', '快捷键', 'OCR', '字幕库'])
    expect(tabs[0].attributes('aria-selected')).toBe('true')
    expect(tabs[0].attributes('aria-controls')).toBe('settings-panel-translation')
    expect(wrapper.get('#settings-panel-translation').attributes('aria-labelledby')).toBe(
      'settings-tab-translation',
    )
  })

  it('按原型为每个一级导航提供独立的图标容器', () => {
    const wrapper = mount(SettingsShell)

    expect(wrapper.findAll('.nav-icon')).toHaveLength(4)
  })

  it('按原型使用二十像素的内容区顶部留白', async () => {
    const source = await readFile(
      resolve(process.cwd(), 'src/components/settings/SettingsShell.vue'),
      'utf8',
    )

    expect(source).toMatch(/\.settings-content \{[^}]*padding: 20px 22px;/)
  })

  it('支持切换一级导航并保持 tab 与 panel 的无障碍关系', async () => {
    const wrapper = mount(SettingsShell)
    const shortcutTab = wrapper.get('#settings-tab-shortcuts')

    await shortcutTab.trigger('click')

    expect(shortcutTab.attributes('aria-selected')).toBe('true')
    expect(wrapper.get('#settings-panel-shortcuts').isVisible()).toBe(true)
  })

  it('从设置页打开主面板', async () => {
    const wrapper = mount(SettingsShell)

    await wrapper.get('button[aria-label="打开主面板"]').trigger('click')

    expect(openMain).toHaveBeenCalledOnce()
  })

  it('支持方向键切换导航并用 Escape 隐藏设置窗口', async () => {
    const wrapper = mount(SettingsShell)

    await wrapper.get('[role="tablist"]').trigger('keydown', { key: 'ArrowDown' })
    expect(wrapper.get('#settings-tab-shortcuts').attributes('aria-selected')).toBe('true')

    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))
    expect(hideCurrent).toHaveBeenCalledOnce()
  })

  it('不展示原型中的模拟成功状态', () => {
    const text = mount(SettingsShell).text()

    expect(text).not.toContain('延迟 240ms')
    expect(text).not.toContain('Vision 引擎就绪')
    expect(text).not.toContain('已节省约 68%')
  })
})
