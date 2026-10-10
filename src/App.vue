<script setup lang="ts">
import { watchEffect, onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useCaptureSource } from '@/composables/useCaptureSource'
import AppProviders from '@/components/AppProviders.vue'
import CaptureFrame from '@/components/CaptureFrame.vue'
import AppToast from '@/components/AppToast.vue'
import MainPanel from '@/components/main-panel/MainPanel.vue'
import { useBootstrap } from '@/composables/useBootstrap'
import { applyBootstrapSnapshot } from '@/composables/useTranslator'

const { selecting, reselect } = useCaptureSource()
const { bootstrap, reloadBootstrap } = useBootstrap()

watchEffect(() => {
  document.body.classList.toggle('selecting', selecting.value)
  if (bootstrap.value.status === 'ready') {
    applyBootstrapSnapshot(bootstrap.value.snapshot)
  }
})

// 悬浮字幕通过 capture:reselect 通知主页重新选择，因为选择状态和 body 样式只存在于主页窗口。
const unlisteners: UnlistenFn[] = []

onMounted(async () => {
  try {
    const fns = await Promise.all([
      listen('capture:reselect', () => reselect()),
    ])
    unlisteners.push(...fns)
  } catch (_) {}
})

onUnmounted(() => {
  unlisteners.forEach(fn => fn())
})
</script>

<template>
  <AppProviders>
    <template v-if="bootstrap.status === 'ready'">
      <CaptureFrame />
      <MainPanel :platform="bootstrap.snapshot.platform.platform" />
      <AppToast />
    </template>
    <main v-else class="bootstrap-state" aria-live="polite">
      <p v-if="bootstrap.status === 'loading'">正在加载本地配置…</p>
      <template v-else>
        <p>{{ bootstrap.error }}</p>
        <button v-if="bootstrap.status === 'database_error'" type="button" @click="reloadBootstrap">
          重试
        </button>
      </template>
    </main>
  </AppProviders>
</template>

<style scoped>
.bootstrap-state {
  min-height: 100vh;
  display: grid;
  place-content: center;
  justify-items: center;
  gap: 12px;
  padding: 24px;
  color: var(--fg);
  text-align: center;
}

.bootstrap-state button {
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 8px 14px;
  background: var(--glass-hi);
  color: inherit;
  cursor: pointer;
}
</style>
