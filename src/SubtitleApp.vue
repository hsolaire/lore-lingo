<script setup lang="ts">
import { ref } from 'vue'
import AppToast from '@/components/AppToast.vue'
import SubtitleBar from '@/components/subtitle/SubtitleBar.vue'
import { useSubtitleBridge } from '@/composables/useSubtitleBridge'

const peekTrigger = ref(0)
function triggerPeek() { peekTrigger.value++ }

useSubtitleBridge(triggerPeek)
</script>

<template>
  <SubtitleBar :external-peek="peekTrigger" />
  <AppToast />
</template>
