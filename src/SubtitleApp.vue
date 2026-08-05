<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import AppToast from '@/components/AppToast.vue'
import SubtitleBar from '@/components/subtitle/SubtitleBar.vue'
import { useSubtitleBridge } from '@/composables/useSubtitleBridge'
import { useBootstrap } from '@/composables/useBootstrap'
import { applyBootstrapSnapshot } from '@/composables/useTranslator'

const peekTrigger = ref(0)
function triggerPeek() { peekTrigger.value++ }

useSubtitleBridge(triggerPeek)
const { bootstrap, reloadBootstrap } = useBootstrap()

watchEffect(() => {
  if (bootstrap.value.status === 'ready') {
    applyBootstrapSnapshot(bootstrap.value.snapshot)
  }
})
</script>

<template>
  <template v-if="bootstrap.status === 'ready'">
    <SubtitleBar :external-peek="peekTrigger" />
    <AppToast />
  </template>
  <main v-else class="bootstrap-state" aria-live="polite">
    <p v-if="bootstrap.status === 'loading'">正在加载悬浮字幕…</p>
    <template v-else>
      <p>{{ bootstrap.error }}</p>
      <button v-if="bootstrap.status === 'database_error'" type="button" @click="reloadBootstrap">
        重试
      </button>
    </template>
  </main>
</template>

<style scoped>
.bootstrap-state {
  min-height: 100vh;
  display: grid;
  place-content: center;
  justify-items: center;
  gap: 8px;
  padding: 16px;
  color: var(--fg);
  text-align: center;
}

.bootstrap-state button {
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 6px 12px;
  background: var(--glass-hi);
  color: inherit;
}
</style>
