import { computed, onMounted, onUnmounted, ref, watch, type Ref } from 'vue'

const PIN_STORAGE_KEY = 'subtitle:toolbar-pinned'
const HIDE_DELAY_MS = 450

export function useOverlayToolbar(keepOpen: Ref<boolean>) {
  const toolbarPinned = ref(false)
  const hovered = ref(false)
  const keyboardFocused = ref(false)
  const keyboardMode = ref(false)
  const peeking = ref(true)
  let timer: ReturnType<typeof setTimeout> | undefined

  try { toolbarPinned.value = localStorage.getItem(PIN_STORAGE_KEY) === 'true' } catch (_) {}

  const visible = computed(() => toolbarPinned.value || hovered.value || keyboardFocused.value || keepOpen.value || peeking.value)

  function scheduleHide(delay = HIDE_DELAY_MS) {
    clearTimeout(timer)
    timer = setTimeout(() => { peeking.value = false }, delay)
  }

  function reveal() {
    clearTimeout(timer)
    hovered.value = true
    peeking.value = true
  }

  function leave() {
    hovered.value = false
    scheduleHide()
  }

  function toggleToolbarPinned() {
    toolbarPinned.value = !toolbarPinned.value
    try { localStorage.setItem(PIN_STORAGE_KEY, String(toolbarPinned.value)) } catch (_) {}
    if (!toolbarPinned.value && !hovered.value) scheduleHide()
  }

  function onKeydown() { keyboardMode.value = true }
  function onPointerdown() { keyboardMode.value = false; keyboardFocused.value = false }
  function onFocusIn() {
    if (keyboardMode.value) keyboardFocused.value = true
  }
  function onFocusOut(event: FocusEvent) {
    if (event.currentTarget instanceof Element && event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget)) return
    keyboardFocused.value = false
    if (!hovered.value) scheduleHide()
  }

  watch(keepOpen, open => {
    if (!open && !hovered.value) scheduleHide()
  })

  // 步骤 1. 首次展示操作提示，并记录键盘与鼠标输入，避免点击按钮后工具栏无法收起。
  onMounted(() => {
    scheduleHide(2200)
    window.addEventListener('keydown', onKeydown, true)
    window.addEventListener('pointerdown', onPointerdown, true)
  })

  // 步骤 2. 销毁窗口时清除计时器和输入监听，防止重复挂载后累积处理器。
  onUnmounted(() => {
    clearTimeout(timer)
    window.removeEventListener('keydown', onKeydown, true)
    window.removeEventListener('pointerdown', onPointerdown, true)
  })

  return { visible, toolbarPinned, reveal, leave, toggleToolbarPinned, onFocusIn, onFocusOut }
}
