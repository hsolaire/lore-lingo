import { ref, watch } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'
import { useToast } from './useToast'

// 单例翻译状态：主面板与悬浮字幕共享原文、译文和语言方向。
const srcText = ref("I never thought I'd find you here, of all places.")
const tgtText = ref('没想到会在这种地方遇见你。')
const langA = ref('English')
const langB = ref('简体中文')
const pinned = ref(false)
/** 是否命中字幕库（true=字幕库命中，false=模型翻译） */
const hit = ref(true)

let isBroadcasting = false

async function broadcastState() {
  if (isBroadcasting) return
  try {
    await emit('translator:update', {
      srcText: srcText.value,
      tgtText: tgtText.value,
      langA: langA.value,
      langB: langB.value,
      pinned: pinned.value,
    })
  } catch (_) {}
}

// Watch all shared state and broadcast on change.
// Guard with isBroadcasting to prevent the subtitle listener
// from re-emitting when it updates local refs.
watch([srcText, tgtText, langA, langB, pinned], () => {
  broadcastState()
})

export type TranslatorPayload = {
  srcText: string
  tgtText: string
  langA: string
  langB: string
  pinned: boolean
}

/**
 * Apply state received from the main window without triggering a re-broadcast.
 * Sets isBroadcasting synchronously around the mutations; the Vue watcher fires
 * asynchronously (nextTick) so the flag is already cleared by then.
 */
export function applyRemoteState(payload: TranslatorPayload) {
  isBroadcasting = true
  srcText.value  = payload.srcText
  tgtText.value  = payload.tgtText
  langA.value    = payload.langA
  langB.value    = payload.langB
  pinned.value   = payload.pinned
  isBroadcasting = false
}

export function useTranslator() {
  const { toast } = useToast()

  /** 翻转语言方向（主面板与悬浮字幕共用）。 */
  function flip() {
    ;[langA.value, langB.value] = [langB.value, langA.value]
    toast('已切换翻译方向')
  }

  /** 交换原文/译文文本（悬浮字幕翻转时同步内容）。 */
  function swapText() {
    ;[srcText.value, tgtText.value] = [tgtText.value, srcText.value]
  }

  /** 切换主窗口置顶；悬浮字幕使用独立的窗口状态。 */
  async function togglePin() {
    const nextState = !pinned.value
    try {
      await getCurrentWindow().setAlwaysOnTop(nextState)
      pinned.value = nextState
    } catch (_) {}
    toast(pinned.value ? '窗口已置顶' : '取消置顶')
  }

  return { srcText, tgtText, langA, langB, pinned, hit, flip, swapText, togglePin }
}
