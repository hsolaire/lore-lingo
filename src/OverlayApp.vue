<script setup lang="ts">
import { ref } from 'vue'
import AppToast from '@/components/AppToast.vue'
import LyricBar from '@/components/lyric/LyricBar.vue'
import { useOverlayBridge } from '@/composables/useOverlayBridge'

const peekTrigger = ref(0)
function triggerPeek() { peekTrigger.value++ }

useOverlayBridge(triggerPeek)
</script>

<template>
  <LyricBar :external-peek="peekTrigger" />
  <AppToast />
</template>
