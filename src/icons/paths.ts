// 逐字拷贝自 index-t.html 的 lucide 风格 SVG 内层标记，保证像素级还原。
// 部分图标使用非默认描边宽度（2.2 / 2.4），随 icon 一并记录。

export interface IconDef {
  /** <svg> 内层标记（path/circle/rect …），由 Icon.vue 经 v-html 注入 */
  body: string
  /** 覆盖默认 stroke-width（2） */
  strokeWidth?: number
  /** 覆盖默认 stroke（用于品牌 logo 的白色描边） */
  stroke?: string
}

export const icons: Record<string, IconDef> = {
  // 品牌 logo · 放大镜（白色描边，2.2）
  search: {
    body: '<circle cx="11" cy="11" r="7"/><path d="m21 21-4.3-4.3"/>',
    strokeWidth: 2.2,
    stroke: '#fff',
  },
  // 标题栏 · 切换歌词条（音符）
  music: {
    body: '<path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/>',
  },
  // 标题栏 · 置顶（图钉）
  pin: {
    body: '<path d="M12 17v5"/><path d="M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H8a2 2 0 0 0 0 4 1 1 0 0 1 1 1z"/>',
  },
  // 标题栏 · 设置（齿轮）
  settings: {
    body: '<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>',
  },
  // 捕获应用（显示器）
  monitor: {
    body: '<rect width="18" height="14" x="3" y="4" rx="2"/><path d="M3 10h18"/><path d="M7 7h.01"/>',
  },
  // 框选区域（虚线取景框）
  scan: {
    body: '<path d="M5 3a2 2 0 0 0-2 2"/><path d="M19 3a2 2 0 0 1 2 2"/><path d="M21 19a2 2 0 0 1-2 2"/><path d="M5 21a2 2 0 0 1-2-2"/><path d="M9 3h1"/><path d="M9 21h1"/><path d="M14 3h1"/><path d="M14 21h1"/><path d="M3 9v1"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M21 14v1"/>',
  },
  // 重选（四角取景框）
  crop: {
    body: '<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/>',
  },
  // 语言方向翻转（左右箭头）
  swap: {
    body: '<path d="m16 3 4 4-4 4"/><path d="M20 7H4"/><path d="m8 21-4-4 4-4"/><path d="M4 17h16"/>',
  },
  // OCR 原文标签（扫描文字）
  scanText: {
    body: '<path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/>',
  },
  // 复制
  copy: {
    body: '<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>',
  },
  // 命中角标 / toast · 对勾（2.4）
  check: {
    body: '<path d="M20 6 9 17l-5-5"/>',
    strokeWidth: 2.4,
  },
  // 设置 Tab · 翻译
  languages: {
    body: '<path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/>',
  },
  // 设置 Tab · OCR（取景框 + 文字线）
  scanLine: {
    body: '<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><path d="M7 12h10"/>',
  },
  // 设置 Tab · 字幕库（字幕）
  captions: {
    body: '<rect width="18" height="14" x="3" y="5" rx="2"/><path d="M7 15h4"/><path d="M15 15h2"/><path d="M7 11h2"/><path d="M13 11h4"/>',
  },
  // 显示/隐藏 Key（眼睛）
  eye: {
    body: '<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/>',
  },
  // 测试连接（对勾圆环）
  checkCircle: {
    body: '<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="m9 11 3 3L22 4"/>',
  },
  // 字幕拖放（下载云）
  upload: {
    body: '<path d="M12 13v8"/><path d="m8 17 4 4 4-4"/><path d="M20 16.7A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25"/>',
  },
  // 移除 / 关闭（X）
  x: {
    body: '<path d="M18 6 6 18"/><path d="m6 6 12 12"/>',
  },
  // 命中优先策略（对勾圆）
  checkRound: {
    body: '<path d="M9 12l2 2 4-4"/><circle cx="12" cy="12" r="10"/>',
  },
  // 回到面板（布局）
  layout: {
    body: '<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M9 21V9"/>',
  },
  // 下拉箭头
  chevronDown: {
    body: '<path d="m6 9 6 6 6-6"/>',
  },
}

export type IconName = keyof typeof icons
