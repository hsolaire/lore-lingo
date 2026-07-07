<script setup lang="ts">
import { useToast } from '@/composables/useToast'
import Icon from '@/components/ui/Icon.vue'

const { message, visible } = useToast()
</script>

<template>
  <div class="toast" :class="{ show: visible }">
    <Icon name="check" />
    <span>{{ message }}</span>
  </div>
</template>

<style scoped>
.toast {
  position: fixed; bottom: 26px; left: 50%; transform: translateX(-50%) translateY(20px);
  background: var(--glass-strong); backdrop-filter: blur(30px); border: 1px solid var(--border-hi);
  color: var(--fg); padding: 10px 16px; border-radius: 999px; font: 500 12px/1 var(--font);
  box-shadow: var(--shadow); opacity: 0; pointer-events: none; transition: .28s; z-index: 50;
  display: inline-flex; align-items: center; gap: 8px;
}
.toast.show { opacity: 1; transform: translateX(-50%) translateY(0); }
.toast :deep(svg) { width: 15px; height: 15px; color: var(--ok); }
</style>
