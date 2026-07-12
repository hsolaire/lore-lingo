<script setup lang="ts">
import { watchEffect, onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useCaptureSource } from '@/composables/useCaptureSource'
import { useTranslator } from '@/composables/useTranslator'
import CaptureFrame from '@/components/CaptureFrame.vue'
import AppToast from '@/components/AppToast.vue'
import MainPanel from '@/components/main-panel/MainPanel.vue'

const { selecting, reselect } = useCaptureSource()
const { togglePin } = useTranslator()

watchEffect(() => {
  document.body.classList.toggle('selecting', selecting.value)
})

// Fix I-1: overlay emits 'capture:reselect' because useCaptureSource's
// selecting ref and body class live in the main window's process.
// Fix I-2: overlay emits 'pin:toggle' because getCurrentWindow() in the
// overlay would pin the overlay, not the main window.
const unlisteners: UnlistenFn[] = []

onMounted(async () => {
  try {
    const fns = await Promise.all([
      listen('capture:reselect', () => reselect()),
      listen('pin:toggle', () => togglePin()),
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
