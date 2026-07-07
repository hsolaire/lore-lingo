<script setup lang="ts">
// 示意：被翻译应用画面上的框选区域。显隐由 body.selecting 控制（见 global.css）。
</script>

<template>
  <div class="capture-frame" aria-hidden="true">
    <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>
  </div>
</template>

<style scoped>
.capture-frame {
  position: fixed;
  left: 50%; top: 46px;
  transform: translateX(-58%);
  width: 520px; height: 118px;
  border: 1.5px dashed var(--accent);
  border-radius: 10px;
  background: oklch(72% 0.14 220 / 0.06);
  box-shadow: 0 0 0 2000px oklch(12% 0.02 260 / 0.28);
  z-index: 1;
  pointer-events: none;
  display: none;
}
.capture-frame::before {
  content: "捕获区域 · Now Playing";
  position: absolute; top: -22px; left: 0;
  font: 500 11px/1 var(--mono);
  letter-spacing: 0.08em;
  color: var(--accent);
}
.corner {
  position: absolute; width: 12px; height: 12px;
  border: 2px solid var(--accent);
}
.corner.tl { top: -1px; left: -1px; border-right: 0; border-bottom: 0; }
.corner.tr { top: -1px; right: -1px; border-left: 0; border-bottom: 0; }
.corner.bl { bottom: -1px; left: -1px; border-right: 0; border-top: 0; }
.corner.br { bottom: -1px; right: -1px; border-left: 0; border-top: 0; }
</style>
