import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import ShortcutBindingsPanel from '@/components/settings/ShortcutBindingsPanel.vue'

describe('快捷键设置面板', () => {
  it('将数据库动作标识转换为简体中文产品文案', () => {
    const wrapper = mount(ShortcutBindingsPanel, {
      props: {
        available: false,
        shortcuts: [
          {
            action: 'translate_once',
            inputKind: 'keyboard',
            bindingKey: 'CommandOrControl+Shift+KeyT',
            binding: null,
            isEnabled: true,
          },
        ],
      },
    })

    expect(wrapper.text()).toContain('翻译当前画面')
    expect(wrapper.text()).not.toContain('translate_once')
  })

  it('在 macOS 上将规范化键盘绑定显示为原生快捷键符号', () => {
    const wrapper = mount(ShortcutBindingsPanel, {
      props: {
        available: false,
        platform: 'macos',
        shortcuts: [
          {
            action: 'translate_once',
            inputKind: 'keyboard',
            bindingKey: 'CommandOrControl+Shift+KeyT',
            binding: null,
            isEnabled: true,
          },
        ],
      } as never,
    })

    expect(wrapper.text()).toContain('⌘⇧T')
    expect(wrapper.text()).not.toContain('CommandOrControl')
  })

  it('在 Windows 上将规范化键盘绑定显示为可读组合键', () => {
    const wrapper = mount(ShortcutBindingsPanel, {
      props: {
        available: false,
        platform: 'windows',
        shortcuts: [
          {
            action: 'translate_once',
            inputKind: 'keyboard',
            bindingKey: 'CommandOrControl+Shift+KeyT',
            binding: null,
            isEnabled: true,
          },
        ],
      } as never,
    })

    expect(wrapper.text()).toContain('Ctrl+Shift+T')
    expect(wrapper.text()).not.toContain('CommandOrControl')
  })
})
