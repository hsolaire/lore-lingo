<script setup lang="ts">
import { CircleAlert, Keyboard } from 'lucide-vue-next'
import { NEmpty, NIcon, NTag } from 'naive-ui'
import type { DeepReadonly } from 'vue'

import type { PlatformKind } from '@/types/desktop'
import type { ShortcutBindingSummary } from '@/types/settings'

const props = withDefaults(defineProps<{
  shortcuts: DeepReadonly<ShortcutBindingSummary[]>
  available: boolean
  platform?: PlatformKind
}>(), {
  platform: 'unsupported',
})

const actionNames: Record<string, string> = {
  translate: '翻译当前画面',
  translate_once: '翻译当前画面',
  auto_start: '开始自动翻译',
  auto_stop: '停止自动翻译',
  toggle_subtitle: '显示或隐藏悬浮字幕',
}

const macKeyNames: Record<string, string> = {
  CommandOrControl: '⌘',
  Command: '⌘',
  Meta: '⌘',
  Control: '⌃',
  Shift: '⇧',
  Alt: '⌥',
  Option: '⌥',
  ArrowUp: '↑',
  ArrowDown: '↓',
  ArrowLeft: '←',
  ArrowRight: '→',
  Space: '空格',
}

const windowsKeyNames: Record<string, string> = {
  CommandOrControl: 'Ctrl',
  Command: 'Win',
  Meta: 'Win',
  Control: 'Ctrl',
  Shift: 'Shift',
  Alt: 'Alt',
  Option: 'Alt',
  Space: '空格',
}

function formatKeyboardToken(token: string) {
  const keyNames = props.platform === 'macos' ? macKeyNames : windowsKeyNames
  if (keyNames[token]) return keyNames[token]
  if (/^Key[A-Z]$/.test(token)) return token.slice(3)
  if (/^Digit\d$/.test(token)) return token.slice(5)
  return token
}

function formatBinding(shortcut: DeepReadonly<ShortcutBindingSummary>) {
  if (!shortcut.bindingKey) return '未绑定'
  if (shortcut.inputKind !== 'keyboard') return shortcut.bindingKey

  const tokens = shortcut.bindingKey.split('+').map(formatKeyboardToken)
  return props.platform === 'macos' ? tokens.join('') : tokens.join('+')
}
</script>

<template>
  <section class="settings-section" aria-labelledby="shortcuts-heading">
    <div class="section-heading">
      <div>
        <h2 id="shortcuts-heading">快捷键</h2>
        <p>查看已保存的键盘或手柄绑定。</p>
      </div>
      <NTag size="small" :type="available ? 'success' : 'warning'" round>
        {{ available ? '系统能力可用' : '系统能力未接入' }}
      </NTag>
    </div>

    <div v-if="shortcuts.length" class="shortcut-list">
      <article v-for="shortcut in shortcuts" :key="shortcut.action" class="shortcut-row">
        <NIcon :component="Keyboard" size="18" />
        <div>
          <b>{{ actionNames[shortcut.action] ?? shortcut.action }}</b>
          <span>{{ shortcut.inputKind === 'gamepad' ? '手柄' : '键盘' }}</span>
        </div>
        <kbd>{{ formatBinding(shortcut) }}</kbd>
      </article>
    </div>
    <NEmpty v-else description="本地数据库中还没有快捷键绑定" />

    <div class="honest-state" role="status">
      <NIcon :component="CircleAlert" size="17" />
      <span>快捷键录入、冲突检测和全局注册尚未实现；当前页面不会伪造保存成功。</span>
    </div>
  </section>
</template>

<style scoped>
.settings-section { display: flex; flex-direction: column; gap: 18px; }
.section-heading { display: flex; justify-content: space-between; align-items: flex-start; gap: 12px; }
.section-heading h2 { font: 650 19px/1.2 var(--font-display); }
.section-heading p { margin-top: 5px; color: var(--faint); font-size: 12px; }
.shortcut-list { display: flex; flex-direction: column; gap: 8px; }
.shortcut-row { display: grid; grid-template-columns: 22px 1fr auto; align-items: center; gap: 10px; padding: 12px; border: 1px solid var(--border); border-radius: 10px; background: var(--surface-muted); }
.shortcut-row > div { display: flex; flex-direction: column; gap: 3px; }
.shortcut-row b { font-size: 12px; }
.shortcut-row span { color: var(--faint); font-size: 10px; }
kbd { min-width: 70px; padding: 6px 9px; border: 1px solid var(--border-hi); border-radius: 7px; background: var(--glass-hi); color: var(--muted); font: 550 11px/1 var(--mono); text-align: center; }
.honest-state { display: flex; align-items: flex-start; gap: 9px; padding: 12px; border: 1px solid color-mix(in oklch, var(--warn) 30%, transparent); border-radius: 10px; background: color-mix(in oklch, var(--warn) 10%, transparent); color: var(--muted); font-size: 11px; line-height: 1.5; }
</style>
