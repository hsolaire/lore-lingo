<script setup lang="ts">
import { useTranslator } from '@/composables/useTranslator'
import { useToast } from '@/composables/useToast'
import Icon from '@/components/ui/Icon.vue'

const props = defineProps<{
  variant: 'source' | 'target'
}>()

const { srcText, tgtText, hit } = useTranslator()
const { toast } = useToast()

function copy() {
  const txt = props.variant === 'source' ? srcText.value : tgtText.value
  if (navigator.clipboard) navigator.clipboard.writeText(txt).catch(() => {})
  toast('已复制' + (props.variant === 'source' ? '原文' : '译文'))
}
</script>

<template>
  <div class="card" :class="variant">
    <div class="lbl">
      <template v-if="variant === 'source'">
        <Icon name="scanText" class="lbl-ico" />
        OCR 原文
      </template>
      <template v-else>
        译文
        <span class="hit" :class="{ model: !hit }">
          <Icon name="check" />{{ hit ? '字幕库命中' : '模型翻译' }}
        </span>
      </template>
    </div>
    <button class="copy" :aria-label="variant === 'source' ? '复制原文' : '复制译文'" @click="copy">
      <Icon name="copy" />
    </button>
    <p class="txt">{{ variant === 'source' ? srcText : tgtText }}</p>
  </div>
</template>

<style scoped>
.card {
  border: 1px solid var(--border); border-radius: 12px;
  background: oklch(24% 0.02 260 / 0.45);
  padding: 13px 14px;
  position: relative;
}
.lbl {
  font: 500 10px/1 var(--mono); letter-spacing: 0.12em;
  color: var(--faint); text-transform: uppercase; margin-bottom: 8px;
  display: flex; align-items: center; gap: 7px;
}
.lbl-ico { width: 12px; height: 12px; }
.card.source .txt { font-size: 15px; line-height: 1.5; color: var(--muted); }
.card.target {
  background: linear-gradient(135deg, var(--accent-dim), oklch(24% 0.02 260 / 0.45) 60%);
  border-color: oklch(72% 0.14 220 / 0.3);
}
.card.target .txt { font: 550 19px/1.45 var(--font-display); color: var(--fg); letter-spacing: -0.01em; }

.hit {
  font: 500 10px/1 var(--mono); letter-spacing: 0.06em;
  color: var(--ok); background: var(--ok-dim); padding: 3px 7px; border-radius: 999px;
  display: inline-flex; align-items: center; gap: 5px; margin-left: auto;
}
.hit :deep(svg) { width: 11px; height: 11px; }
.hit.model { color: var(--accent); background: var(--accent-dim); }

.copy {
  position: absolute; top: 11px; right: 11px;
  width: 26px; height: 26px; border-radius: 7px; border: 1px solid var(--border);
  background: var(--glass-hi); color: var(--muted); cursor: pointer; display: grid;
  place-items: center; opacity: 0; transition: .15s;
}
.card:hover .copy { opacity: 1; }
.copy:hover { color: var(--accent); border-color: var(--accent); }
.copy :deep(svg) { width: 13px; height: 13px; }
</style>
