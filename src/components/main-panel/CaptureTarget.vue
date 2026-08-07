<script setup lang="ts">
import { RefreshCw } from 'lucide-vue-next'
import { useCaptureSource } from '@/composables/useCaptureSource'

const {
  targetName,
  targetMeta,
  selectedWindow,
  windowList,
  listOpen,
  listLoading,
  toggleList,
  selectWindow,
  fetchWindows,
} = useCaptureSource()

function fallbackLetter(label: string) {
  return label.charAt(0).toUpperCase()
}
</script>

<template>
  <div class="capture-target-wrap">
    <!-- 当前选中的窗口 -->
    <div class="target" :class="{ open: listOpen }" @click="toggleList" role="button" tabindex="0" @keydown.enter="toggleList">
      <div class="thumb" v-if="selectedWindow?.iconDataUrl">
        <img :src="selectedWindow.iconDataUrl" width="26" height="26" style="border-radius:6px;display:block" />
      </div>
      <div class="thumb letter" v-else>
        {{ selectedWindow ? fallbackLetter(selectedWindow.applicationName ?? selectedWindow.title) : '未' }}
      </div>
      <div style="flex:1;min-width:0">
        <b>{{ targetName }}</b>
        <div class="dim">{{ targetMeta }}</div>
      </div>
      <div class="chevron" :class="{ up: listOpen }">›</div>
    </div>

    <!-- 窗口来源列表 -->
    <div class="win-list" v-if="listOpen">
      <div class="list-header">
        <span class="list-title">选择窗口</span>
        <button class="refresh-btn" @click.stop="fetchWindows" :disabled="listLoading" title="刷新">
          <RefreshCw :size="12" :stroke-width="2.5" :class="{ spin: listLoading }" />
        </button>
      </div>

      <div class="list-empty" v-if="!listLoading && windowList.length === 0">
        未找到可捕获的窗口
      </div>

      <div
        v-for="w in windowList"
        :key="w.sourceId"
        class="win-row"
        :class="{ selected: selectedWindow?.sourceId === w.sourceId }"
        @click="selectWindow(w)"
      >
        <div class="win-icon">
          <img v-if="w.iconDataUrl" :src="w.iconDataUrl" width="20" height="20" style="display:block" />
          <span v-else class="fallback-letter">{{ fallbackLetter(w.applicationName ?? w.title) }}</span>
        </div>
        <div class="win-info">
          <div class="win-title">{{ w.title }}</div>
          <div class="win-exe">{{ w.applicationName ?? '未知应用' }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.capture-target-wrap {
  margin: 8px 14px 0;
  position: relative;
}

.target {
  padding: 8px 11px;
  display: flex; align-items: center; gap: 9px;
  background: oklch(20% 0.02 260 / 0.4);
  border: 1px solid var(--border); border-radius: 10px;
  font-size: 12px; color: var(--muted);
  cursor: pointer; user-select: none;
  transition: border-color .15s;
}
.target:hover, .target.open { border-color: var(--accent); }
.target b { color: var(--fg); font-weight: 550; }
.dim { color: var(--faint); font: 400 11px/1 var(--mono); }

.thumb {
  width: 26px; height: 26px; border-radius: 6px; flex: none;
  background: linear-gradient(135deg, oklch(55% 0.11 25), oklch(48% 0.09 300));
  display: grid; place-items: center; font: 600 12px/1 var(--font); color: #fff;
  overflow: hidden;
}
.thumb.letter { background: linear-gradient(135deg, oklch(55% 0.11 25), oklch(48% 0.09 300)); }

.chevron {
  color: var(--faint); font-size: 14px; line-height: 1;
  transform: rotate(90deg); transition: transform .2s;
  margin-left: auto; flex: none;
}
.chevron.up { transform: rotate(-90deg); }

/* 下拉列表悬浮在内容上方，避免撑开主面板。 */
.win-list {
  position: absolute;
  top: calc(100% + 4px); left: 0; right: 0;
  z-index: 20;
  background: oklch(18% 0.02 260 / 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid var(--border); border-radius: 10px;
  box-shadow: 0 12px 32px oklch(10% 0.02 260 / 0.55);
  overflow: hidden; max-height: 240px; overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: var(--border-hi) transparent;
}
.win-list::-webkit-scrollbar { width: 6px; }
.win-list::-webkit-scrollbar-track { background: transparent; }
.win-list::-webkit-scrollbar-thumb {
  background: var(--border-hi); border-radius: 3px;
}
.win-list::-webkit-scrollbar-thumb:hover { background: var(--faint); }

.list-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 8px 10px 6px; border-bottom: 1px solid var(--border);
}
.list-title { font: 500 11px/1 var(--font); color: var(--muted); }
.refresh-btn {
  background: none; border: none; cursor: pointer; color: var(--muted);
  padding: 2px; border-radius: 4px; display: grid; place-items: center;
  transition: color .15s;
}
.refresh-btn:hover { color: var(--accent); }
.refresh-btn:disabled { opacity: 0.4; cursor: default; }

.list-empty {
  padding: 16px 12px; text-align: center;
  font: 400 11px/1 var(--font); color: var(--faint);
}

.win-row {
  display: flex; align-items: center; gap: 8px;
  padding: 7px 10px; cursor: pointer; transition: background .12s;
}
.win-row:hover { background: oklch(26% 0.03 260 / 0.6); }
.win-row.selected { background: oklch(30% 0.05 260 / 0.5); }

.win-icon {
  width: 20px; height: 20px; flex: none;
  display: grid; place-items: center;
}
.fallback-letter {
  width: 20px; height: 20px; border-radius: 4px;
  background: linear-gradient(135deg, oklch(50% 0.1 220), oklch(44% 0.08 280));
  display: grid; place-items: center;
  font: 600 10px/1 var(--font); color: #fff;
}

.win-info { flex: 1; min-width: 0; }
.win-title {
  font: 500 12px/1.3 var(--font); color: var(--fg);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.win-exe { font: 400 10px/1 var(--mono); color: var(--faint); margin-top: 2px; }

@keyframes spin { to { transform: rotate(360deg); } }
.spin { animation: spin .7s linear infinite; }
</style>
