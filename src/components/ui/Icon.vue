<script setup lang="ts">
import { computed } from 'vue'
import { icons, type IconName } from '@/icons/paths'

const props = defineProps<{
  name: IconName
  /** 覆盖 icon 定义的描边宽度 */
  strokeWidth?: number
}>()

const def = computed(() => icons[props.name])
const stroke = computed(() => def.value.stroke ?? 'currentColor')
const width = computed(() => props.strokeWidth ?? def.value.strokeWidth ?? 2)
</script>

<template>
  <svg
    viewBox="0 0 24 24"
    fill="none"
    :stroke="stroke"
    :stroke-width="width"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
    v-html="def.body"
  />
</template>
