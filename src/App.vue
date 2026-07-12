<script setup lang="ts">
import { watchEffect } from 'vue'
import { useCaptureSource } from '@/composables/useCaptureSource'
import CaptureFrame from '@/components/CaptureFrame.vue'
import AppToast from '@/components/AppToast.vue'
import MainPanel from '@/components/main-panel/MainPanel.vue'

const { selecting } = useCaptureSource()

watchEffect(() => {
  document.body.classList.toggle('selecting', selecting.value)
})
</script>

<template>
  <CaptureFrame />
  <MainPanel />
  <AppToast />
</template>
