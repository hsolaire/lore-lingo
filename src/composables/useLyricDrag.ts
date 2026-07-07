import { onMounted, onUnmounted, reactive, ref, type Ref } from 'vue'

const STORAGE_KEY = 'lyric:pos'

function clamp(v: number, min: number, max: number) {
  return Math.max(min, Math.min(max, v))
}

/**
 * 歌词条拖拽：按住「歌词框本体」拖动整条到屏幕任意位置，
 * 位置持久化到 localStorage，窗口尺寸变化时拉回可视范围。
 * mousemove/mouseup/resize 挂在 window，onMounted 添加 / onUnmounted 移除。
 *
 * @param barEl  歌词条根元素（定位对象）
 * @param onDrop 拖拽结束（发生位移）后的回调，用于触发 peek
 */
export function useLyricDrag(barEl: Ref<HTMLElement | null>, onDrop?: () => void) {
  const placed = ref(false)
  const dragging = ref(false)
  const pos = reactive({ x: 0, y: 0 })

  let drag: { dx: number; dy: number; moved: boolean } | null = null

  function applyPos(x: number, y: number) {
    const el = barEl.value
    if (!el) return
    const w = el.offsetWidth
    const h = el.offsetHeight
    pos.x = clamp(x, 8, window.innerWidth - w - 8)
    pos.y = clamp(y, 8, window.innerHeight - h - 8)
    placed.value = true
  }

  /** 由 LyricBox 的 @mousedown 调用——仅框体本身启动拖拽，避免与控制排按钮冲突 */
  function onBoxMousedown(e: MouseEvent) {
    if (e.button !== 0) return
    const el = barEl.value
    if (!el) return
    const r = el.getBoundingClientRect()
    drag = { dx: e.clientX - r.left, dy: e.clientY - r.top, moved: false }
    dragging.value = true
    e.preventDefault()
  }

  function onMove(e: MouseEvent) {
    if (!drag) return
    drag.moved = true
    applyPos(e.clientX - drag.dx, e.clientY - drag.dy)
  }

  function onUp() {
    if (!drag) return
    dragging.value = false
    if (drag.moved) {
      try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify({ x: pos.x, y: pos.y }))
      } catch (_) {
        /* localStorage 不可用时静默 */
      }
      onDrop?.()
    }
    drag = null
  }

  function onResize() {
    if (placed.value) applyPos(pos.x, pos.y)
  }

  function restore() {
    try {
      const p = JSON.parse(localStorage.getItem(STORAGE_KEY) || 'null')
      if (p && typeof p.x === 'number') applyPos(p.x, p.y)
    } catch (_) {
      /* 忽略损坏的持久化值 */
    }
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

  return { placed, dragging, pos, onBoxMousedown }
}
