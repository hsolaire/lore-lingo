<script setup lang="ts">
// 复用 .field > label + .select 表单块
// options 支持纯字符串或 {value, label} 对象两种形式
export type SelectOption = string | { value: string; label: string }

const model = defineModel<string>()

defineProps<{
  label: string
  options: SelectOption[]
}>()

function optionValue(opt: SelectOption) {
  return typeof opt === 'string' ? opt : opt.value
}
function optionLabel(opt: SelectOption) {
  return typeof opt === 'string' ? opt : opt.label
}
</script>

<template>
  <div class="field">
    <label>{{ label }}</label>
    <select class="select" v-model="model">
      <option
        v-for="opt in options"
        :key="optionValue(opt)"
        :value="optionValue(opt)"
      >{{ optionLabel(opt) }}</option>
    </select>
  </div>
</template>

<style scoped>
.field { display: flex; flex-direction: column; gap: 6px; }
.field > label { font: 500 11px/1 var(--font); color: var(--muted); letter-spacing: 0.01em; }
.select {
  width: 100%; background: oklch(18% 0.02 260 / 0.6); color: var(--fg);
  border: 1px solid var(--border); border-radius: 9px; padding: 10px 12px;
  font: 400 13px/1.2 var(--font); transition: .15s; outline: none;
  appearance: none; cursor: pointer; padding-right: 34px;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2399a' stroke-width='2.5'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat; background-position: right 12px center;
}
.select:focus { border-color: var(--accent); box-shadow: 0 0 0 3px var(--accent-dim); }
</style>
