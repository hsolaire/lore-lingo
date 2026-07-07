<script setup lang="ts">
import { useLyricMode } from '@/composables/useLyricMode'
import { useTranslator } from '@/composables/useTranslator'
import { useCaptureSource } from '@/composables/useCaptureSource'
import Icon from '@/components/ui/Icon.vue'

const emit = defineEmits<{ flip: [] }>()

const { exit, close } = useLyricMode()
const { pinned, togglePin } = useTranslator()
const { reselect } = useCaptureSource()
</script>

<template>
  <div class="lyric-ctrl">
    <span class="cnow"><span class="dot"></span>VLC · 实时</span>
    <button class="cbtn" title="重选捕获" aria-label="重选捕获" @click="reselect">
      <Icon name="crop" />
    </button>
    <button class="cbtn" title="切换翻译方向" aria-label="切换翻译方向" @click="emit('flip')">
      <Icon name="swap" />
    </button>
    <button
      class="cbtn"
      :class="{ active: pinned }"
      title="窗口置顶"
      aria-label="窗口置顶"
      @click="togglePin"
    >
      <Icon name="pin" />
    </button>
    <span class="cdiv"></span>
    <button class="cbtn" title="回到面板" aria-label="回到面板视图" @click="exit">
      <Icon name="layout" />
    </button>
    <button class="cbtn danger" title="关闭歌词条" aria-label="关闭歌词条" @click="close">
      <Icon name="x" />
    </button>
  </div>
</template>

<style scoped>
.lyric-ctrl {
  -webkit-app-region: no-drag;
  display: flex; align-items: center; gap: 6px;
  align-self: center;
  margin-bottom: 8px;
  padding: 5px;
  background: var(--glass-strong);
  backdrop-filter: blur(30px) saturate(1.6);
  -webkit-backdrop-filter: blur(30px) saturate(1.6);
  border: 1px solid var(--border-hi);
  border-radius: 999px;
  box-shadow: 0 12px 30px oklch(12% 0.02 260 / 0.5);
  opacity: 0; transform: translateY(6px);
  transition: opacity .22s ease, transform .22s ease;
  pointer-events: none;
}
.cbtn {
  -webkit-app-region: no-drag;
  width: 32px; height: 32px; border-radius: 999px;
  border: 1px solid transparent; background: transparent; color: var(--muted);
  display: grid; place-items: center; cursor: pointer; transition: .15s;
}
.cbtn:hover { background: var(--glass-hi); color: var(--fg); }
.cbtn.active { color: var(--accent); background: var(--accent-dim); }
.cbtn.danger:hover { color: oklch(66% 0.17 25); background: oklch(66% 0.17 25 / 0.16); }
.cbtn :deep(svg) { width: 16px; height: 16px; }
.cdiv { width: 1px; height: 18px; background: var(--border-hi); margin: 0 2px; }
.cnow {
  font: 500 11px/1 var(--font); color: var(--muted); letter-spacing: 0.01em;
  padding: 0 10px 0 8px; display: inline-flex; align-items: center; gap: 6px; white-space: nowrap;
}
.cnow .dot { width: 6px; height: 6px; border-radius: 50%; background: var(--ok);
  box-shadow: 0 0 0 0 var(--ok); animation: blink 1.8s ease-in-out infinite; flex: none; }
</style>
