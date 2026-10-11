<script setup lang="ts">
import { computed, type DeepReadonly } from 'vue'
import { RotateCcw, X } from 'lucide-vue-next'
import { NIcon } from 'naive-ui'

import type { PlatformKind } from '@/types/desktop'
import type { ShortcutBindingSummary } from '@/types/settings'

const props = withDefaults(defineProps<{
  shortcuts: DeepReadonly<ShortcutBindingSummary[]>
  available: boolean
  platform?: PlatformKind
}>(), {
  platform: 'unsupported',
})

const shortcutDefinitions = [
  {
    action: 'translate_once',
    aliases: ['translate_once', 'translate'],
    title: '翻译',
    description: '手动翻译当前识别到的英文',
  },
  {
    action: 'auto_start',
    aliases: ['auto_start'],
    title: '自动翻译',
    description: '开始按间隔持续翻译',
  },
  {
    action: 'auto_stop',
    aliases: ['auto_stop'],
    title: '停止自动翻译',
    description: '停止持续识别与翻译',
  },
] as const

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

const shortcutGroups = computed(() => shortcutDefinitions.map((definition) => ({
  ...definition,
  keyboard: props.shortcuts.find((shortcut) => (
    definition.aliases.some((action) => action === shortcut.action)
    && shortcut.inputKind === 'keyboard'
  )),
  gamepad: props.shortcuts.find((shortcut) => (
    definition.aliases.some((action) => action === shortcut.action)
    && shortcut.inputKind === 'gamepad'
  )),
})))

function formatKeyboardToken(token: string) {
  const keyNames = props.platform === 'macos' ? macKeyNames : windowsKeyNames
  if (keyNames[token]) return keyNames[token]
  if (/^Key[A-Z]$/.test(token)) return token.slice(3)
  if (/^Digit\d$/.test(token)) return token.slice(5)
  return token
}

function formatBinding(shortcut?: DeepReadonly<ShortcutBindingSummary>) {
  if (!shortcut?.bindingKey) return '未绑定'
  if (shortcut.inputKind !== 'keyboard') return shortcut.bindingKey

  const tokens = shortcut.bindingKey.split('+').map(formatKeyboardToken)
  return props.platform === 'macos' ? tokens.join('') : tokens.join('+')
}
</script>

<template>
  <section class="settings-section" aria-labelledby="shortcuts-heading">
    <div class="section-heading">
      <h2 id="shortcuts-heading">快捷键</h2>
      <p>查看已保存的绑定；键盘与手柄录入暂不可用。</p>
    </div>

    <div class="shortcut-list">
      <article v-for="group in shortcutGroups" :key="group.action" class="shortcut-item">
        <span class="shortcut-copy">
          <b>{{ group.title }}</b>
          <em>{{ group.description }}</em>
        </span>

        <div class="bind-grid">
          <button
            type="button"
            class="bind-btn"
            :aria-label="`设置${group.title}的键盘快捷键`"
            title="快捷键录入尚未接入"
            disabled
          >
            <span>键盘</span>
            <b>{{ formatBinding(group.keyboard) }}</b>
          </button>
          <button
            type="button"
            class="bind-btn"
            :aria-label="`设置${group.title}的手柄快捷键`"
            title="手柄按键录入尚未接入"
            disabled
          >
            <span>手柄</span>
            <b>{{ formatBinding(group.gamepad) }}</b>
          </button>
        </div>

        <div class="bind-tools">
          <button
            type="button"
            class="bind-tool"
            :aria-label="`恢复默认${group.title}快捷键`"
            title="快捷键保存尚未接入"
            disabled
          >
            <NIcon :component="RotateCcw" size="12" />
            恢复默认
          </button>
          <button
            type="button"
            class="bind-tool danger"
            :aria-label="`清除${group.title}快捷键`"
            title="快捷键保存尚未接入"
            disabled
          >
            <NIcon :component="X" size="12" />
            清除
          </button>
        </div>
      </article>
    </div>

    <p class="binding-status" :class="{ warn: !available }" role="status">
      <span class="status-dot" aria-hidden="true" />
      <span>
        快捷键录入与保存尚未接入；{{ available ? '当前仅展示已保存绑定。' : '全局快捷键能力当前不可用。' }}
      </span>
    </p>
  </section>
</template>

<style scoped>
.settings-section { display: flex; flex-direction: column; gap: 14px; }
.section-heading { margin-bottom: 16px; }
.section-heading h2 { color: var(--fg); font: 600 17px/1.2 var(--font-display); letter-spacing: -.01em; }
.section-heading p { margin-top: 4px; color: var(--faint); font: 400 12px/1.4 var(--font); }
.shortcut-list { display: flex; flex-direction: column; gap: 9px; }
.shortcut-item {
  display: grid; grid-template-columns: minmax(0, 1fr) 220px; column-gap: 12px; row-gap: 8px;
  align-items: center; padding: 11px 12px; border: 1px solid var(--border); border-radius: 10px;
  background: color-mix(in oklch, var(--surface-muted) 78%, transparent);
}
.shortcut-copy { min-width: 0; grid-row: 1 / span 2; }
.shortcut-copy b { display: block; color: var(--fg); font: 550 13px/1.25 var(--font); }
.shortcut-copy em { display: block; margin-top: 3px; color: var(--faint); font: 400 11px/1.4 var(--font); font-style: normal; }
.bind-grid { grid-column: 2; display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }
.bind-tools { grid-column: 2; display: flex; gap: 6px; }
.bind-btn,
.bind-tool { opacity: 1; -webkit-appearance: none; appearance: none; }
.bind-btn {
  display: flex; min-width: 0; flex-direction: column; align-items: center; justify-content: center; gap: 4px;
  padding: 8px 9px; border: 1px solid var(--border); border-radius: 9px;
  background: color-mix(in oklch, var(--surface-muted) 48%, transparent); color: var(--fg);
}
.bind-btn span { color: var(--faint); font: 500 10px/1 var(--font); letter-spacing: .02em; }
.bind-btn b { max-width: 100%; overflow: hidden; color: var(--fg); font: 600 11px/1 var(--mono); letter-spacing: .02em; text-overflow: ellipsis; white-space: nowrap; }
.bind-tool {
  display: inline-flex; flex: 1; align-items: center; justify-content: center; gap: 4px;
  padding: 5px 6px; border: 1px solid var(--border); border-radius: 8px;
  background: color-mix(in oklch, var(--surface-muted) 42%, transparent); color: var(--faint);
  font: 500 10px/1 var(--font); letter-spacing: .02em;
}
.bind-btn:disabled,
.bind-tool:disabled { cursor: not-allowed; }
.binding-status { display: flex; align-items: flex-start; gap: 8px; margin-top: 1px; color: var(--faint); font: 400 11px/1.45 var(--font); }
.status-dot { width: 7px; height: 7px; flex: none; margin-top: 4px; border-radius: 50%; background: var(--ok); }
.binding-status.warn .status-dot { background: var(--warn); }
@media (max-width: 600px) {
  .shortcut-item { grid-template-columns: 1fr; }
  .shortcut-copy { grid-row: auto; }
  .bind-grid,
  .bind-tools { grid-column: 1; }
}
</style>
