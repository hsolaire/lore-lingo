<script setup lang="ts">
import { ref } from 'vue'
import ToggleSwitch from '@/components/ui/ToggleSwitch.vue'

const autoTranslate = ref(true)
</script>

<template>
  <div class="toolbar">
    <ToggleSwitch v-model="autoTranslate">自动翻译</ToggleSwitch>
    <span class="spacer"></span>
    <span class="kbd">⌥⇧ T</span>
  </div>
</template>

<style scoped>
.toolbar {
  display: flex; align-items: center; gap: 8px;
  padding: 12px 14px 14px; margin-top: 4px;
}
.spacer { flex: 1; }
.kbd { font: 500 10px/1 var(--mono); color: var(--faint); letter-spacing: 0.04em;
  border: 1px solid var(--border); border-radius: 6px; padding: 4px 6px; background: var(--glass-hi); }
</style>
