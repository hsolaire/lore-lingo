<script setup lang="ts">
import { useTranslator } from '@/composables/useTranslator'
import Icon from '@/components/ui/Icon.vue'

const { langA, langB, flip } = useTranslator()
</script>

<template>
  <div class="lang-flip">
    <span class="pill">{{ langA }}</span>
    <button title="切换方向" aria-label="切换翻译方向" @click="flip">
      <Icon name="swap" />
    </button>
    <span class="pill">{{ langB }}</span>
  </div>
</template>

<style scoped>
.lang-flip {
  display: flex; align-items: center; justify-content: center; gap: 10px;
  font: 500 11px/1 var(--font); color: var(--faint);
  padding: 2px 0;
}
.pill { color: var(--muted); letter-spacing: 0.02em; }
button {
  width: 24px; height: 24px; border-radius: 7px; border: 1px solid var(--border);
  background: var(--glass-hi); color: var(--muted); cursor: pointer; display: grid;
  place-items: center; transition: .18s;
}
button:hover { color: var(--accent); border-color: var(--accent); transform: rotate(180deg); }
button :deep(svg) { width: 12px; height: 12px; }
</style>
