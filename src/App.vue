<script setup lang="ts">
import { watchEffect, onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useCaptureSource } from '@/composables/useCaptureSource'
import CaptureFrame from '@/components/CaptureFrame.vue'
import AppToast from '@/components/AppToast.vue'
import MainPanel from '@/components/main-panel/MainPanel.vue'

const { selecting, reselect } = useCaptureSource()

watchEffect(() => {
  document.body.classList.toggle('selecting', selecting.value)
})

// The subtitle window emits capture:reselect because this state and body class
// selecting ref and body class live in the main window's process.
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
  <CaptureFrame />
  <MainPanel />
  <AppToast />
</template>
