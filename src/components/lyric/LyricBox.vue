<script setup lang="ts">
import { useTranslator } from '@/composables/useTranslator'
import Icon from '@/components/ui/Icon.vue'

defineProps<{
  /** 翻转动画进行中：原文/译文淡出 */
  swap?: boolean
}>()

const { srcText, tgtText, hit } = useTranslator()
</script>

<template>
  <div class="lyric-box" :class="{ swap }">
    <span class="lyric-hit" :class="{ model: !hit }">
      <Icon name="check" />{{ hit ? '字幕库命中' : '模型翻译' }}
    </span>
    <p class="lyric-src">{{ srcText }}</p>
    <p class="lyric-tgt">{{ tgtText }}</p>
  </div>
</template>

<style scoped>
.lyric-box {
  -webkit-app-region: drag;
  position: relative;
  background: oklch(20% 0.02 260 / 0.62);
  backdrop-filter: blur(44px) saturate(1.8);
  -webkit-backdrop-filter: blur(44px) saturate(1.8);
  border: 1px solid var(--border-hi);
  border-radius: 18px;
  box-shadow: var(--shadow);
  padding: 18px 26px 20px;
  text-align: center;
  overflow: hidden;
}
.lyric-box::before { content: ""; position: absolute; inset: 0 0 auto 0; height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-hi), transparent); }
.lyric-src {
  font: 500 16px/1.45 var(--font); color: var(--muted); letter-spacing: 0.01em;
  margin-bottom: 7px; transition: opacity .18s;
}
.lyric-tgt {
  font: 600 30px/1.35 var(--font-display); color: var(--fg); letter-spacing: -0.015em;
  text-wrap: balance; transition: opacity .18s;
}
.lyric-box.swap .lyric-src, .lyric-box.swap .lyric-tgt { opacity: 0; }
.lyric-hit {
  position: absolute; top: 12px; right: 14px;
  font: 500 9px/1 var(--mono); letter-spacing: 0.06em;
  color: var(--ok); background: var(--ok-dim); padding: 4px 8px; border-radius: 999px;
  display: inline-flex; align-items: center; gap: 5px; opacity: .9;
}
.lyric-hit :deep(svg) { width: 10px; height: 10px; }
.lyric-hit.model { color: var(--accent); background: var(--accent-dim); }
</style>
