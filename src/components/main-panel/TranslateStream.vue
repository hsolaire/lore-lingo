<script setup lang="ts">
import LangFlip from './LangFlip.vue'
import TranslationCard from './TranslationCard.vue'
</script>

<template>
  <div class="stream">
    <LangFlip />
    <TranslationCard variant="source" />
    <TranslationCard variant="target" />
  </div>
</template>

<style scoped>
.stream { padding: 12px 14px 4px; display: flex; flex-direction: column; gap: 10px; }
</style>
