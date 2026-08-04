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
.stream {
  flex: 1 1 auto;
  min-height: 96px;
  padding: 12px 14px 4px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: var(--border-hi) transparent;
}
.stream::-webkit-scrollbar { width: 8px; }
.stream::-webkit-scrollbar-thumb {
  background: var(--border-hi);
  border: 2px solid transparent;
  border-radius: 8px;
  background-clip: padding-box;
}
.stream :deep(.card) { flex: 1 0 auto; }
.stream :deep(.card.source) { min-height: 92px; }
.stream :deep(.card.target) { min-height: 106px; }
</style>
