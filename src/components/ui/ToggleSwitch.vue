<script setup lang="ts">
// 复用 .toggle/.switch 开关（工具栏「自动翻译」、OCR「去重连续相同帧」）
const model = defineModel<boolean>({ default: false })

defineProps<{
  /** flex-start 布局（OCR 面板内使用） */
  start?: boolean
}>()
</script>

<template>
  <label class="toggle" :style="start ? { justifyContent: 'flex-start' } : undefined">
    <input type="checkbox" v-model="model" />
    <span class="switch"></span>
    <slot />
  </label>
</template>

<style scoped>
.toggle {
  display: inline-flex; align-items: center; gap: 8px;
  font-size: 12px; color: var(--muted); cursor: pointer; user-select: none;
}
.switch {
  width: 34px; height: 20px; border-radius: 999px; background: oklch(40% 0.02 260 / 0.6);
  border: 1px solid var(--border); position: relative; transition: .2s;
}
.switch::after {
  content: ""; position: absolute; top: 2px; left: 2px; width: 14px; height: 14px;
  border-radius: 50%; background: var(--fg); transition: .2s;
}
.toggle input { display: none; }
.toggle input:checked + .switch { background: var(--accent); border-color: var(--accent); }
.toggle input:checked + .switch::after { left: 16px; background: var(--accent-fg); }
</style>
