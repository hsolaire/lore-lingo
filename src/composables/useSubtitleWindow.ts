import { onMounted, onUnmounted, ref, type Ref } from 'vue'
import {
  getSubtitleAlwaysOnTop,
  resizeSubtitle,
  setSubtitleAlwaysOnTop,
  startSubtitleDrag,
  startSubtitleResize,
} from '@/services/subtitleWindow'

const WIDTH_STORAGE_KEY = 'subtitle:width'
const MIN_WINDOW_WIDTH = 520
const MAX_WINDOW_WIDTH = 1248
const RESIZE_SETTLE_MS = 80

const pinned = ref(true)

function clamp(value: number, min: number, max: number) {
  return Math.max(min, Math.min(max, value))
}

function readStoredWidth() {
  try {
    const currentWidth = Number(localStorage.getItem(WIDTH_STORAGE_KEY))
    if (Number.isFinite(currentWidth) && currentWidth > 0) return currentWidth
  } catch (_) {}
  return null
}

export function useSubtitleWindow(contentEl: Ref<HTMLElement | null>) {
  let observer: ResizeObserver | null = null
  let resizeTimer: ReturnType<typeof setTimeout> | undefined
  let lastAppliedHeight = 0
  let disposed = false

  function fitWindowToContent() {
    clearTimeout(resizeTimer)
    resizeTimer = setTimeout(() => {
      const el = contentEl.value
      if (!el) return

      const height = Math.max(104, Math.min(720, Math.ceil(el.getBoundingClientRect().height)))
      if (Math.abs(height - lastAppliedHeight) <= 1) return

      lastAppliedHeight = height
      void resizeSubtitle(window.innerWidth, height).catch(() => {})
    }, RESIZE_SETTLE_MS)
  }

  function persistWidth() {
    try {
      localStorage.setItem(WIDTH_STORAGE_KEY, String(Math.round(window.innerWidth)))
    } catch (_) {}
  }

  function onWindowResize() {
    persistWidth()
    fitWindowToContent()
  }

  function onDragBarMousedown(event: MouseEvent) {
    if (event.button !== 0) return
    event.preventDefault()
    void startSubtitleDrag().catch(() => {})
  }

  function onResizeMousedown(event: MouseEvent, side: 'left' | 'right') {
    if (event.button !== 0) return
    event.preventDefault()
    event.stopPropagation()
    void startSubtitleResize(side).catch(() => {})
  }

  async function togglePinned() {
    const nextPinned = !pinned.value
    try {
      await setSubtitleAlwaysOnTop(nextPinned)
      pinned.value = nextPinned
    } catch (_) {}
  }

  async function initialize() {
    try {
      pinned.value = await getSubtitleAlwaysOnTop()
    } catch (_) {}

    const storedWidth = readStoredWidth()
    if (storedWidth) {
      const width = clamp(storedWidth, MIN_WINDOW_WIDTH, MAX_WINDOW_WIDTH)
      if (disposed) return
      await resizeSubtitle(width, window.innerHeight).catch(() => {})
    }

    if (disposed) return
    if (typeof ResizeObserver === 'undefined') return
    observer = new ResizeObserver(fitWindowToContent)
    if (contentEl.value) observer.observe(contentEl.value)
    window.addEventListener('resize', onWindowResize)
    fitWindowToContent()
  }

  onMounted(() => {
    void initialize()
  })

  onUnmounted(() => {
    disposed = true
    clearTimeout(resizeTimer)
    observer?.disconnect()
    window.removeEventListener('resize', onWindowResize)
  })

  return {
    pinned,
    onDragBarMousedown,
    onResizeMousedown,
    togglePinned,
  }
}
