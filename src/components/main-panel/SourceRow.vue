<script setup lang="ts">
import { useCaptureSource } from '@/composables/useCaptureSource'
import Icon from '@/components/ui/Icon.vue'

const { sourceMode, setSource, reselect } = useCaptureSource()
</script>

<template>
  <div class="source-row">
    <div class="seg" role="tablist">
      <button
        class="seg-btn"
        :class="{ on: sourceMode === 'app' }"
        role="tab"
        :aria-selected="sourceMode === 'app'"
        @click="setSource('app')"
      >
        <Icon name="monitor" />
        捕获应用
      </button>
      <button
        class="seg-btn"
        :class="{ on: sourceMode === 'region' }"
        role="tab"
        :aria-selected="sourceMode === 'region'"
        @click="setSource('region')"
      >
        <Icon name="scan" />
        框选区域
      </button>
    </div>
    <button class="reselect" @click="reselect">
      <Icon name="crop" />
      重选
    </button>
  </div>
</template>

<style scoped>
.source-row {
  display: flex; gap: 8px; padding: 12px 14px 4px;
}
.seg {
  flex: 1; display: flex; background: oklch(20% 0.02 260 / 0.5);
  border: 1px solid var(--border); border-radius: 10px; padding: 3px;
}
.seg-btn {
  flex: 1; border: 0; background: transparent; color: var(--muted);
  font: 500 12px/1 var(--font); padding: 8px 6px; border-radius: 7px;
  cursor: pointer; transition: .18s; display: inline-flex; align-items: center;
  justify-content: center; gap: 6px; letter-spacing: 0;
}
.seg-btn :deep(svg) { width: 14px; height: 14px; }
.seg-btn.on { background: var(--glass-strong); color: var(--fg);
  box-shadow: 0 1px 3px oklch(10% 0.02 260 / 0.4); }
.reselect {
  -webkit-app-region: no-drag;
  border: 1px solid var(--border); background: var(--glass-hi); color: var(--muted);
  border-radius: 10px; padding: 0 12px; font: 500 12px/1 var(--font); cursor: pointer;
  display: inline-flex; align-items: center; gap: 6px; transition: .15s;
}
.reselect:hover { color: var(--accent); border-color: var(--accent); }
.reselect :deep(svg) { width: 14px; height: 14px; }
</style>
