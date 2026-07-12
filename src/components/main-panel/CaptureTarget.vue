<script setup lang="ts">
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

function iconSrc(b64: string) {
  return `data:image/png;base64,${b64}`
}

function fallbackLetter(exe: string) {
  return exe.charAt(0).toUpperCase()
}
</script>

<template>
  <div class="capture-target-wrap">
    <!-- Selected window card (clickable) -->
    <div class="target" :class="{ open: listOpen }" @click="toggleList" role="button" tabindex="0" @keydown.enter="toggleList">
      <div class="thumb" v-if="selectedWindow && selectedWindow.icon_b64">
        <img :src="iconSrc(selectedWindow.icon_b64)" width="26" height="26" style="border-radius:6px;display:block" />
      </div>
      <div class="thumb letter" v-else>
        {{ selectedWindow ? fallbackLetter(selectedWindow.exe) : '未' }}
      </div>
      <div style="flex:1;min-width:0">
        <b>{{ targetName }}</b>
        <div class="dim">{{ targetMeta }}</div>
      </div>
      <div class="chevron" :class="{ up: listOpen }">›</div>
    </div>

    <!-- Dropdown list -->
    <div class="win-list" v-if="listOpen">
      <div class="list-header">
        <span class="list-title">选择窗口</span>
        <button class="refresh-btn" @click.stop="fetchWindows" :disabled="listLoading" title="刷新">
          <svg v-if="!listLoading" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M23 4v6h-6M1 20v-6h6"/>
            <path d="M3.51 9a9 9 0 0114.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0020.49 15"/>
          </svg>
          <svg v-else width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="spin">
            <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
          </svg>
        </button>
      </div>

      <div class="list-empty" v-if="!listLoading && windowList.length === 0">
        未找到可捕获的窗口
      </div>

      <div
        v-for="w in windowList"
        :key="w.id"
        class="win-row"
        :class="{ selected: selectedWindow?.id === w.id }"
        @click="selectWindow(w)"
      >
        <div class="win-icon">
          <img v-if="w.icon_b64" :src="iconSrc(w.icon_b64)" width="20" height="20" style="display:block" />
          <span v-else class="fallback-letter">{{ fallbackLetter(w.exe) }}</span>
        </div>
        <div class="win-info">
          <div class="win-title">{{ w.title }}</div>
          <div class="win-exe">{{ w.exe }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.capture-target-wrap {
  margin: 8px 14px 0;
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

/* Dropdown */
.win-list {
  margin-top: 4px;
  background: oklch(18% 0.02 260 / 0.95);
  border: 1px solid var(--border); border-radius: 10px;
  overflow: hidden; max-height: 240px; overflow-y: auto;
}

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
