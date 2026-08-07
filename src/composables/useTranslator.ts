import { ref, watch, type DeepReadonly } from 'vue'
import { emit } from '@tauri-apps/api/event'
import { useToast } from './useToast'
import type { BootstrapSnapshot } from '@/types/app'
import { currentWindowClient } from '@/services/windowClient'

// 单例翻译状态：主面板与悬浮字幕共享原文、译文和语言方向。
const srcText = ref('')
const tgtText = ref('')
const langA = ref('auto')
const langB = ref('zh-CN')
const pinned = ref(false)
/** 是否命中字幕库（true=字幕库命中，false=模型翻译） */
const hit = ref(false)
const sessionActive = ref(false)

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
      hit: hit.value,
      sessionActive: sessionActive.value,
    })
  } catch (_) {}
}

// 监听共享状态变化；远端状态写入期间通过标记阻止悬浮字幕再次广播。
watch([srcText, tgtText, langA, langB, pinned, hit, sessionActive], () => {
  broadcastState()
})

export type TranslatorPayload = {
  srcText: string
  tgtText: string
  langA: string
  langB: string
  pinned: boolean
  hit: boolean
  sessionActive: boolean
}

/** 应用主窗口发来的状态，同时避免触发反向广播。 */
export function applyRemoteState(payload: TranslatorPayload) {
  isBroadcasting = true
  srcText.value  = payload.srcText
  tgtText.value  = payload.tgtText
  langA.value    = payload.langA
  langB.value    = payload.langB
  pinned.value   = payload.pinned
  hit.value = payload.hit
  sessionActive.value = payload.sessionActive
  isBroadcasting = false
}

export function applyBootstrapSnapshot(snapshot: DeepReadonly<BootstrapSnapshot>) {
  // 步骤 1. 使用持久化设置恢复语言方向，不保留原型中的示例语言或模型状态
  isBroadcasting = true
  langA.value = snapshot.settings.translation.sourceLanguage
  langB.value = snapshot.settings.translation.targetLanguage

  // 步骤 2. 仅在存在真实历史结果时恢复原文、译文和结果来源
  srcText.value = snapshot.latestResult?.sourceText ?? ''
  tgtText.value = snapshot.latestResult?.translatedText ?? ''
  hit.value = snapshot.latestResult?.resultSource === 'subtitle'

  // 步骤 3. 活动状态只依据 Rust 快照，不使用硬编码窗口名称或成功状态
  sessionActive.value = snapshot.activeSession !== null
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
      await currentWindowClient.setAlwaysOnTop(nextState)
      pinned.value = nextState
    } catch (_) {}
    toast(pinned.value ? '窗口已置顶' : '取消置顶')
  }

  return {
    srcText,
    tgtText,
    langA,
    langB,
    pinned,
    hit,
    sessionActive,
    flip,
    swapText,
    togglePin,
  }
}
