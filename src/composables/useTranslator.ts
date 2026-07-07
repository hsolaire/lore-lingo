import { ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useToast } from './useToast'

// 单例翻译状态：面板与歌词条共享同一份原文/译文/语言方向/置顶。
const srcText = ref("I never thought I'd find you here, of all places.")
const tgtText = ref('没想到会在这种地方遇见你。')
const langA = ref('English')
const langB = ref('简体中文')
const pinned = ref(false)
/** 是否命中字幕库（true=字幕库命中，false=模型翻译） */
const hit = ref(true)

export function useTranslator() {
  const { toast } = useToast()

  /** 翻转语言方向（面板与歌词条共用） */
  function flip() {
    ;[langA.value, langB.value] = [langB.value, langA.value]
    toast('已切换翻译方向')
  }

  /** 交换原文/译文文本（歌词条翻转时同步内容） */
  function swapText() {
    ;[srcText.value, tgtText.value] = [tgtText.value, srcText.value]
  }

  /** 切换置顶：同步到 Tauri 原生窗口层级 */
  async function togglePin() {
    pinned.value = !pinned.value
    await getCurrentWindow().setAlwaysOnTop(pinned.value)
    toast(pinned.value ? '窗口已置顶' : '取消置顶')
  }

  return { srcText, tgtText, langA, langB, pinned, hit, flip, swapText, togglePin }
}
