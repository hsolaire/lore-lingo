import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { currentWindowClient } from '@/services/windowClient'
import { applyRemoteState, type TranslatorPayload } from './useTranslator'

export function useSubtitleBridge(onPeek: () => void) {
  const unlisten: UnlistenFn[] = []

  onMounted(async () => {
    // 步骤 1. 并行注册跨窗口事件，避免初始化过程中出现部分事件已生效、部分尚未生效的状态。
    const fns = await Promise.all([
      // 步骤 2. 使用远端状态入口更新响应式数据，阻止悬浮字幕窗口把同一状态反向广播。
      listen<TranslatorPayload>('translator:update', ({ payload }) => {
        applyRemoteState(payload)
      }),
      listen('subtitle:show', () => { onPeek() }),
      listen('subtitle:peek', () => { onPeek() }),
      // 步骤 3. 隐藏和关闭请求都复用当前窗口客户端，保持 Tauri 调用集中在 services 层。
      listen('subtitle:hide', () => currentWindowClient.hide()),
      listen('subtitle:close', () => currentWindowClient.hide()),
    ])
    unlisten.push(...fns)
  })

  onUnmounted(() => {
    unlisten.forEach(fn => fn())
  })
}
