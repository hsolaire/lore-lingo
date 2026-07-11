<script setup lang="ts">
import { ref } from 'vue'
import { useCaptureSource } from '@/composables/useCaptureSource'
import Icon from '@/components/ui/Icon.vue'

const {
  sourceMode, loadingWindows,
  windows, selectedWindow,
  targetName, targetMeta,
  selectWindow,
} = useCaptureSource()

const open = ref(false)

function pick(w: typeof windows.value[number]) {
  selectWindow(w)
  open.value = false
}
</script>

<template>
  <!-- 已选窗口 / 区域信息展示 -->
  <div
    class="target"
    :class="{ clickable: sourceMode === 'app' && !loadingWindows, open }"
    @click="sourceMode === 'app' && !loadingWindows && (open = !open)"
  >
    <span class="thumb">
      <Icon v-if="loadingWindows" name="scan" class="spin" />
      <template v-else>{{ (selectedWindow?.appName ?? targetName).charAt(0).toUpperCase() }}</template>
    </span>
    <div style="flex:1;min-width:0">
      <b>{{ targetName }}</b>
      <div class="dim">{{ targetMeta }}</div>
    </div>
    <Icon v-if="sourceMode === 'app'" name="chevronDown" class="chevron" />
  </div>

  <!-- app 模式：下拉列表 -->
  <div v-if="sourceMode === 'app'" class="dropdown" :class="{ open }">
    <div class="dropdown-inner">
      <div v-if="loadingWindows" class="list-placeholder">
        <Icon name="scan" class="spin" /> 正在获取窗口列表…
      </div>
      <div v-else-if="windows.length === 0" class="list-placeholder">
        未找到可捕获的窗口
      </div>
      <button
        v-else
        v-for="w in windows"
        :key="w.id"
        class="win-item"
        :class="{ active: selectedWindow?.id === w.id }"
        @click="pick(w)"
      >
        <span class="win-icon">{{ w.appName.charAt(0).toUpperCase() }}</span>
        <div class="win-meta">
          <b>{{ w.title }}</b>
          <span>{{ w.appName }} · {{ w.width }}×{{ w.height }}</span>
        </div>
        <Icon v-if="selectedWindow?.id === w.id" name="check" class="win-check" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.target {
  margin: 8px 14px 0; padding: 8px 11px;
  display: flex; align-items: center; gap: 9px;
  background: oklch(20% 0.02 260 / 0.4);
  border: 1px solid var(--border); border-radius: 10px;
  font-size: 12px; color: var(--muted);
  transition: border-color .15s, background .15s;
}
.target.clickable { cursor: pointer; }
.target.clickable:hover { background: oklch(22% 0.02 260 / 0.5); border-color: var(--border-hi); }
.target.open { border-color: var(--accent); border-bottom-left-radius: 0; border-bottom-right-radius: 0; }

.chevron {
  width: 14px; height: 14px; color: var(--faint); flex: none;
  transition: transform .2s cubic-bezier(.4,0,.2,1);
}
.open .chevron { transform: rotate(180deg); }

.thumb {
  width: 26px; height: 26px; border-radius: 6px; flex: none;
  background: linear-gradient(135deg, oklch(55% 0.11 25), oklch(48% 0.09 300));
  display: grid; place-items: center; font: 600 12px/1 var(--font); color: #fff;
}
.thumb :deep(svg) { width: 14px; height: 14px; }
.target b { color: var(--fg); font-weight: 550; }
.dim { color: var(--faint); font: 400 11px/1 var(--mono); margin-top: 2px; }

/* 下拉容器 */
.dropdown {
  margin: 0 14px;
  display: grid;
  grid-template-rows: 0fr;
  transition: grid-template-rows .22s cubic-bezier(.4,0,.2,1);
  border: 1px solid transparent;
  border-top: none;
  border-radius: 0 0 10px 10px;
}
.dropdown.open {
  grid-template-rows: 1fr;
  border-color: var(--accent);
  border-top: none;
}
.dropdown-inner {
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 3px;
  background: oklch(12% 0.02 260 / 0.85);
  border-radius: 0 0 10px 10px;
}
/* 内部滚动限高 */
.dropdown.open .dropdown-inner {
  max-height: 200px;
  overflow-y: auto;
  padding: 4px;
  scrollbar-width: thin;
  scrollbar-color: var(--border) transparent;
}

.list-placeholder {
  display: flex; align-items: center; gap: 8px;
  padding: 12px; font: 400 12px/1 var(--font); color: var(--faint);
  justify-content: center;
}
.list-placeholder :deep(svg) { width: 14px; height: 14px; }

.win-item {
  display: flex; align-items: center; gap: 9px;
  padding: 7px 10px; border-radius: 7px; border: 1px solid transparent;
  background: transparent; cursor: pointer; text-align: left;
  transition: .15s; width: 100%; color: var(--muted);
}
.win-item:hover { background: var(--glass-hi); border-color: var(--border); color: var(--fg); }
.win-item.active { background: var(--accent-dim); border-color: oklch(72% 0.14 220 / 0.3); color: var(--fg); }

.win-icon {
  width: 24px; height: 24px; border-radius: 6px; flex: none;
  background: oklch(30% 0.03 260 / 0.8);
  display: grid; place-items: center;
  font: 600 11px/1 var(--font); color: var(--muted);
}
.win-item.active .win-icon { background: var(--accent-dim); color: var(--accent); }

.win-meta { flex: 1; min-width: 0; }
.win-meta b {
  display: block; font: 500 12px/1.3 var(--font); color: inherit;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.win-meta span { font: 400 10px/1 var(--mono); color: var(--faint); }

.win-check { width: 14px; height: 14px; color: var(--accent); flex: none; }

@keyframes spin { to { transform: rotate(360deg); } }
.spin { animation: spin 1s linear infinite; }
</style>
