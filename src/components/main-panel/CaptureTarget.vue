<script setup lang="ts">
import { useCaptureSource } from '@/composables/useCaptureSource'

const { targetName, targetMeta } = useCaptureSource()
</script>

<template>
  <div class="target">
    <span class="thumb">V</span>
    <div style="flex:1;min-width:0">
      <b>{{ targetName }}</b>
      <div class="dim">{{ targetMeta }}</div>
    </div>
  </div>
</template>

<style scoped>
.target {
  margin: 8px 14px 0; padding: 8px 11px;
  display: flex; align-items: center; gap: 9px;
  background: oklch(20% 0.02 260 / 0.4);
  border: 1px solid var(--border); border-radius: 10px;
  font-size: 12px; color: var(--muted);
}
.thumb {
  width: 26px; height: 26px; border-radius: 6px; flex: none;
  background: linear-gradient(135deg, oklch(55% 0.11 25), oklch(48% 0.09 300));
  display: grid; place-items: center; font: 600 12px/1 var(--font); color: #fff;
}
.target b { color: var(--fg); font-weight: 550; }
.dim { color: var(--faint); font: 400 11px/1 var(--mono); }
</style>
