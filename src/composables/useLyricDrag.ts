import { onMounted, onUnmounted, reactive, ref, type Ref } from 'vue'

const STORAGE_KEY = 'lyric:pos'
const SIZE_KEY = 'lyric:size'

function clamp(v: number, min: number, max: number) {
  return Math.max(min, Math.min(max, v))
}

export function useLyricDrag(barEl: Ref<HTMLElement | null>, onDrop?: () => void) {
  const placed = ref(false)
  const dragging = ref(false)
  const pos = reactive({ x: 0, y: 0 })
  const size = reactive({ width: 0 })   // 0 = 未手动调整，用 CSS 默认值

  let drag: { dx: number; dy: number; moved: boolean } | null = null
  let resizeSide: 'left' | 'right' | null = null
  let resizeStartX = 0
  let resizeStartW = 0
  let resizeStartLeft = 0

  // ── 定位 ──────────────────────────────────────────────────
  function applyPos(x: number, y: number) {
    const el = barEl.value
    if (!el) return
    const w = el.offsetWidth
    const h = el.offsetHeight
    pos.x = clamp(x, 8, window.innerWidth - w - 8)
    pos.y = clamp(y, 8, window.innerHeight - h - 8)
    placed.value = true
  }

  /** drag-bar mousedown → 拖动整条 */
  function onDragBarMousedown(e: MouseEvent) {
    if (e.button !== 0) return
    const el = barEl.value
    if (!el) return
    const r = el.getBoundingClientRect()
    drag = { dx: e.clientX - r.left, dy: e.clientY - r.top, moved: false }
    dragging.value = true
    e.preventDefault()
  }

  // ── 拉伸 ──────────────────────────────────────────────────
  function onResizeMousedown(e: MouseEvent, side: 'left' | 'right') {
    if (e.button !== 0) return
    const el = barEl.value
    if (!el) return
    resizeSide = side
    resizeStartX = e.clientX
    resizeStartW = el.offsetWidth
    resizeStartLeft = el.getBoundingClientRect().left
    e.preventDefault()
    e.stopPropagation()
  }

  // ── 公共 mousemove / mouseup ───────────────────────────────
  function onMove(e: MouseEvent) {
    if (drag) {
      drag.moved = true
      applyPos(e.clientX - drag.dx, e.clientY - drag.dy)
      return
    }
    if (resizeSide) {
      const delta = e.clientX - resizeStartX
      const minW = 280
      const maxW = window.innerWidth - 32
      if (resizeSide === 'right') {
        size.width = clamp(resizeStartW + delta, minW, maxW)
      } else {
        // 左手柄：宽度增大方向与鼠标方向相反，同时左移
        const newW = clamp(resizeStartW - delta, minW, maxW)
        size.width = newW
        // 保持右边缘不动
        pos.x = clamp(resizeStartLeft + resizeStartW - newW, 8, window.innerWidth - newW - 8)
        placed.value = true
      }
    }
  }

  function onUp() {
    if (drag) {
      dragging.value = false
      if (drag.moved) {
        savePos()
        onDrop?.()
      }
      drag = null
    }
    if (resizeSide) {
      resizeSide = null
      saveSize()
    }
  }

  function onResize() {
    if (placed.value) applyPos(pos.x, pos.y)
  }

  // ── 持久化 ────────────────────────────────────────────────
  function savePos() {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify({ x: pos.x, y: pos.y })) } catch (_) {}
  }
  function saveSize() {
    try { localStorage.setItem(SIZE_KEY, JSON.stringify({ width: size.width })) } catch (_) {}
  }
  function restore() {
    try {
      const p = JSON.parse(localStorage.getItem(STORAGE_KEY) || 'null')
      if (p && typeof p.x === 'number') applyPos(p.x, p.y)
      const s = JSON.parse(localStorage.getItem(SIZE_KEY) || 'null')
      if (s && typeof s.width === 'number') size.width = s.width
    } catch (_) {}
  }

  onMounted(() => {
    window.addEventListener('mousemove', onMove)
    window.addEventListener('mouseup', onUp)
    window.addEventListener('resize', onResize)
    restore()
  })
  onUnmounted(() => {
    window.removeEventListener('mousemove', onMove)
    window.removeEventListener('mouseup', onUp)
    window.removeEventListener('resize', onResize)
  })

  return { placed, dragging, pos, size, onDragBarMousedown, onResizeMousedown }
}
