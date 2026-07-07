<script setup lang="ts">
import { ref } from 'vue'
import { useSettings } from '@/composables/useSettings'
import Icon from '@/components/ui/Icon.vue'
import TranslatePanel from './TranslatePanel.vue'
import OcrPanel from './OcrPanel.vue'
import SubsPanel from './SubsPanel.vue'

const { drawerOpen } = useSettings()

type Tab = 'translate' | 'ocr' | 'subs'
const activeTab = ref<Tab>('translate')
</script>

<template>
  <div class="drawer" :class="{ open: drawerOpen }">
    <div class="drawer-inner">
      <div class="drawer-scroll">
        <div class="tabs" role="tablist">
          <button :class="{ on: activeTab === 'translate' }" @click="activeTab = 'translate'">
            <Icon name="languages" />
            翻译
          </button>
          <button :class="{ on: activeTab === 'ocr' }" @click="activeTab = 'ocr'">
            <Icon name="scanLine" />
            OCR
          </button>
          <button :class="{ on: activeTab === 'subs' }" @click="activeTab = 'subs'">
            <Icon name="captions" />
            字幕库
          </button>
        </div>

        <TranslatePanel v-show="activeTab === 'translate'" />
        <OcrPanel v-show="activeTab === 'ocr'" />
        <SubsPanel v-show="activeTab === 'subs'" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.drawer {
  display: grid; grid-template-rows: 0fr;
  transition: grid-template-rows .32s cubic-bezier(.4,0,.2,1);
  border-top: 1px solid transparent;
}
.drawer.open { grid-template-rows: 1fr; border-top-color: var(--border); }
.drawer-inner { overflow: hidden; }
.drawer-scroll { max-height: 340px; overflow-y: auto; padding: 4px 14px 14px; }
.drawer-scroll::-webkit-scrollbar { width: 8px; }
.drawer-scroll::-webkit-scrollbar-thumb { background: var(--border-hi); border-radius: 8px; border: 2px solid transparent; background-clip: padding-box; }

.tabs { display: flex; gap: 4px; padding: 12px 0 12px; position: sticky; top: 0;
  background: linear-gradient(var(--glass-strong), var(--glass)); backdrop-filter: blur(20px); z-index: 2; }
.tabs button {
  flex: 1; border: 1px solid transparent; background: transparent; color: var(--muted);
  font: 500 12px/1 var(--font); padding: 8px 4px; border-radius: 8px; cursor: pointer;
  transition: .15s; display: inline-flex; align-items: center; justify-content: center; gap: 6px;
}
.tabs button :deep(svg) { width: 14px; height: 14px; }
.tabs button.on { color: var(--fg); background: var(--glass-hi); border-color: var(--border); }
</style>
