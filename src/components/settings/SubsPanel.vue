<script setup lang="ts">
import { ref } from 'vue'
import { useSubtitles } from '@/composables/useSubtitles'
import Icon from '@/components/ui/Icon.vue'

const { items, addFiles, remove } = useSubtitles()

const fileInput = ref<HTMLInputElement | null>(null)
const drag = ref(false)

function onDrop(ev: DragEvent) {
  ev.preventDefault()
  drag.value = false
  if (ev.dataTransfer?.files) addFiles(ev.dataTransfer.files)
}
function onChange() {
  if (fileInput.value?.files) addFiles(fileInput.value.files)
}
</script>

<template>
  <div class="panel">
    <div
      class="dropzone"
      :class="{ drag }"
      @click="fileInput?.click()"
      @dragover.prevent="drag = true"
      @dragenter.prevent="drag = true"
      @dragleave.prevent="drag = false"
      @drop="onDrop"
    >
      <Icon name="upload" />
      <b>拖入或点击加载字幕</b>
      <span>支持 .srt · .ass · .vtt — 命中后免去模型查找</span>
      <input
        ref="fileInput"
        type="file"
        accept=".srt,.ass,.vtt"
        multiple
        hidden
        @change="onChange"
      />
    </div>

    <TransitionGroup tag="div" class="sub-list" name="sub">
      <div class="sub-item" v-for="item in items" :key="item.id">
        <span class="fmt">{{ item.fmt }}</span>
        <div class="meta"><b>{{ item.name }}</b><span>{{ item.status }}</span></div>
        <button class="rm" aria-label="移除" @click="remove(item.id)">
          <Icon name="x" />
        </button>
      </div>
    </TransitionGroup>

    <div class="strategy">
      <Icon name="checkRound" />
      <p><b>命中优先策略：</b>OCR 文本先在已加载字幕中做模糊匹配，命中即直接采用译文；未命中才调用翻译模型。已节省约 <b>68%</b> 的模型请求。</p>
    </div>
  </div>
</template>

<style scoped>
.panel { display: flex; flex-direction: column; gap: 14px; padding-top: 4px; animation: fade .25s ease; }

.dropzone {
  border: 1.5px dashed var(--border-hi); border-radius: 12px; padding: 18px;
  text-align: center; color: var(--muted); cursor: pointer; transition: .18s;
  background: oklch(20% 0.02 260 / 0.3);
}
.dropzone:hover, .dropzone.drag { border-color: var(--accent); color: var(--fg); background: var(--accent-dim); }
.dropzone :deep(svg) { width: 22px; height: 22px; margin-bottom: 8px; color: var(--faint); }
.dropzone.drag :deep(svg), .dropzone:hover :deep(svg) { color: var(--accent); }
.dropzone b { display: block; font: 550 13px/1.3 var(--font); color: var(--fg); margin-bottom: 3px; }
.dropzone span { font: 400 11px/1.4 var(--font); }

.sub-list { display: flex; flex-direction: column; gap: 7px; }
.sub-enter-active, .sub-leave-active { transition: opacity .18s; }
.sub-enter-from, .sub-leave-to { opacity: 0; }
.sub-item {
  display: flex; align-items: center; gap: 10px; padding: 9px 11px;
  background: oklch(22% 0.02 260 / 0.5); border: 1px solid var(--border); border-radius: 9px;
}
.sub-item .fmt { font: 600 9px/1 var(--mono); letter-spacing: 0.06em; color: var(--accent);
  background: var(--accent-dim); padding: 4px 6px; border-radius: 5px; flex: none; }
.sub-item .meta { flex: 1; min-width: 0; }
.sub-item .meta b { display: block; font: 500 12px/1.3 var(--font); color: var(--fg);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.sub-item .meta span { font: 400 10px/1 var(--mono); color: var(--faint); }
.sub-item .rm { width: 24px; height: 24px; border: 0; background: transparent; color: var(--faint);
  cursor: pointer; border-radius: 6px; display: grid; place-items: center; flex: none; }
.sub-item .rm:hover { color: oklch(66% 0.17 25); background: oklch(66% 0.17 25 / 0.14); }
.sub-item .rm :deep(svg) { width: 14px; height: 14px; }

.strategy { display: flex; align-items: flex-start; gap: 9px; padding: 11px;
  background: var(--ok-dim); border: 1px solid oklch(76% 0.15 155 / 0.28); border-radius: 10px; }
.strategy :deep(svg) { width: 16px; height: 16px; color: var(--ok); flex: none; margin-top: 1px; }
.strategy p { font: 400 11px/1.5 var(--font); color: var(--muted); }
.strategy b { color: var(--fg); font-weight: 550; }
</style>
