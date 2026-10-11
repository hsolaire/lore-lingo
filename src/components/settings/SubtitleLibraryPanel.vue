<script setup lang="ts">
import { Captions, CircleAlert, Database } from 'lucide-vue-next'
import { NButton, NEmpty, NIcon, NTag } from 'naive-ui'
</script>

<template>
  <section class="settings-section" aria-labelledby="subtitle-library-heading">
    <div class="section-heading">
      <div>
        <h2 id="subtitle-library-heading">字幕库</h2>
        <p>管理可用于优先匹配的本地字幕。</p>
      </div>
      <NTag size="small" type="default" round>导入能力未接入</NTag>
    </div>

    <div class="dropzone" aria-disabled="true">
      <NIcon :component="Captions" size="25" />
      <b>字幕导入暂不可用</b>
      <span>计划支持 .srt、.ass、.vtt 格式。</span>
      <NButton size="small" disabled>选择字幕文件</NButton>
    </div>

    <NEmpty description="字幕文件列表暂不可用">
      <template #icon><NIcon :component="Database" /></template>
    </NEmpty>

    <div class="honest-state" role="status">
      <NIcon :component="CircleAlert" size="17" />
      <span>字幕导入、管理与优先匹配尚未接入。目前请使用区域或窗口翻译。</span>
    </div>
  </section>
</template>

<style scoped>
.settings-section { display: flex; flex-direction: column; gap: 18px; }
.section-heading { display: flex; justify-content: space-between; align-items: flex-start; gap: 12px; }
.section-heading h2 { font: 650 19px/1.2 var(--font-display); }
.section-heading p { margin-top: 5px; color: var(--faint); font-size: 12px; }
.dropzone { display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 22px 16px; border: 1.5px dashed var(--border-hi); border-radius: 12px; background: var(--surface-muted); text-align: center; }
.dropzone b { font-size: 13px; }
.dropzone span { max-width: 300px; color: var(--faint); font-size: 11px; line-height: 1.5; }
.honest-state { display: flex; gap: 9px; padding: 12px; border: 1px solid color-mix(in oklch, var(--warn) 30%, transparent); border-radius: 10px; background: color-mix(in oklch, var(--warn) 10%, transparent); color: var(--muted); font-size: 11px; line-height: 1.5; }
</style>
