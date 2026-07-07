<script setup lang="ts">
import { watchEffect } from 'vue'
import { useLyricMode } from '@/composables/useLyricMode'
import { useCaptureSource } from '@/composables/useCaptureSource'
import CaptureFrame from '@/components/CaptureFrame.vue'
import AppToast from '@/components/AppToast.vue'
import MainPanel from '@/components/main-panel/MainPanel.vue'
import LyricBar from '@/components/lyric/LyricBar.vue'

const { lyricMode } = useLyricMode()
const { selecting } = useCaptureSource()

// body-class 副作用：驱动壁纸顶对齐（lyric-mode）与框选提示层显隐（selecting）。
watchEffect(() => {
  document.body.classList.toggle('lyric-mode', lyricMode.value)
})
watchEffect(() => {
  document.body.classList.toggle('selecting', selecting.value)
})
</script>

<template>
  <CaptureFrame />
  <MainPanel v-show="!lyricMode" />
  <LyricBar v-show="lyricMode" />
  <AppToast />
</template>
