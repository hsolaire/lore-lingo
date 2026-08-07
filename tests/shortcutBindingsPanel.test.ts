import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import ShortcutBindingsPanel from '@/components/settings/ShortcutBindingsPanel.vue'
import type { ShortcutBindingSummary } from '@/types/settings'

const shortcuts: ShortcutBindingSummary[] = [
  {
    action: 'translate_once',
    inputKind: 'keyboard',
    bindingKey: 'CommandOrControl+Shift+KeyT',
    binding: null,
    isEnabled: true,
  },
  {
    action: 'translate_once',
    inputKind: 'gamepad',
    bindingKey: null,
    binding: null,
    isEnabled: true,
  },
  {
    action: 'auto_start',
    inputKind: 'keyboard',
    bindingKey: 'CommandOrControl+Shift+KeyA',
    binding: null,
    isEnabled: true,
  },
  {
    action: 'auto_start',
    inputKind: 'gamepad',
    bindingKey: null,
    binding: null,
    isEnabled: true,
  },
  {
    action: 'auto_stop',
    inputKind: 'keyboard',
    bindingKey: 'CommandOrControl+Shift+KeyS',
    binding: null,
    isEnabled: true,
  },
  {
    action: 'auto_stop',
    inputKind: 'gamepad',
    bindingKey: null,
    binding: null,
    isEnabled: true,
  },
]

describe('快捷键设置面板', () => {
  it('按原型把键盘与手柄绑定合并到三个动作卡片中', () => {
    const wrapper = mount(ShortcutBindingsPanel, {
      props: { available: false, platform: 'macos', shortcuts },
    })

    const cards = wrapper.findAll('.shortcut-item')
    expect(cards).toHaveLength(3)
    expect(cards.map((card) => card.find('.shortcut-copy b').text())).toEqual([
      '翻译',
      '自动翻译',
      '停止自动翻译',
    ])
    expect(cards[0].text()).toContain('手动翻译当前识别到的英文')
    expect(cards[0].text()).toContain('键盘')
    expect(cards[0].text()).toContain('⌘⇧T')
    expect(cards[0].text()).toContain('手柄')
    expect(cards[0].text()).toContain('未绑定')
  })

  it('呈现原型中的恢复默认和清除控件，但在写入能力未接入时保持禁用', () => {
    const wrapper = mount(ShortcutBindingsPanel, {
      props: { available: false, platform: 'macos', shortcuts },
    })

    expect(wrapper.findAll('button[aria-label^="恢复默认"]').length).toBe(3)
    expect(wrapper.findAll('button[aria-label^="清除"]').length).toBe(3)
    expect(wrapper.findAll('.bind-tool:disabled').length).toBe(6)
    expect(wrapper.text()).toContain('快捷键录入与保存尚未接入')
    expect(wrapper.text()).not.toContain('系统能力未接入')
  })

  it('保留原型标题区与快捷键卡片列表之间的双层间距', async () => {
    const source = await readFile(
      resolve(process.cwd(), 'src/components/settings/ShortcutBindingsPanel.vue'),
      'utf8',
    )

    expect(source).toMatch(/\.section-heading \{ margin-bottom: 16px; \}/)
  })

  it('在 macOS 上将规范化键盘绑定显示为原生快捷键符号', () => {
    const wrapper = mount(ShortcutBindingsPanel, {
      props: { available: false, platform: 'macos', shortcuts },
    })

    expect(wrapper.text()).toContain('⌘⇧T')
    expect(wrapper.text()).not.toContain('CommandOrControl')
  })

  it('在 Windows 上将规范化键盘绑定显示为可读组合键', () => {
    const wrapper = mount(ShortcutBindingsPanel, {
      props: { available: false, platform: 'windows', shortcuts },
    })

    expect(wrapper.text()).toContain('Ctrl+Shift+T')
    expect(wrapper.text()).not.toContain('CommandOrControl')
  })
})
