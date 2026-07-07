<script setup lang="ts">
import TitleBar from './TitleBar.vue'
import SourceRow from './SourceRow.vue'
import CaptureTarget from './CaptureTarget.vue'
import TranslateStream from './TranslateStream.vue'
import Toolbar from './Toolbar.vue'
import SettingsDrawer from '@/components/settings/SettingsDrawer.vue'
</script>

<template>
  <main class="app">
    <TitleBar />
    <SourceRow />
    <CaptureTarget />
    <TranslateStream />
    <Toolbar />
    <SettingsDrawer />
  </main>
</template>

<style scoped>
.app {
  position: relative;
  z-index: 3;
  width: 420px;
  max-width: 100%;
  background: var(--glass);
  backdrop-filter: blur(40px) saturate(1.7);
  -webkit-backdrop-filter: blur(40px) saturate(1.7);
  border: 1px solid var(--border-hi);
  border-radius: var(--radius);
  box-shadow: var(--shadow);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.app::before { /* 顶部高光 */
  content: ""; position: absolute; inset: 0 0 auto 0; height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-hi), transparent);
}
</style>
