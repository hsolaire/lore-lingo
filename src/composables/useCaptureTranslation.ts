import { computed, readonly, ref, shallowRef } from 'vue'

import { captureTranslationClient } from '@/services/captureTranslationClient'
import type {
  CaptureTranslationEvent,
  CaptureTranslationViewState,
  PublicError,
  TranslationConfig,
} from '@/types/capturedLayout'
import { toPublicError } from '@/types/translation'
import { useTranslator } from './useTranslator'
import type { CaptureSourceSummary } from '@/types/desktop'

const state = shallowRef<CaptureTranslationViewState>({
  status: 'idle',
  sessionId: null,
  document: null,
  error: null,
})
const config = ref<TranslationConfig>({
  endpoint: '',
  model: '',
  apiKey: '',
  sourceLanguage: 'auto',
  targetLanguage: 'zh-CN',
})
const starting = ref(false)
let listening: Promise<() => void> | null = null

function publicError(reason: unknown): PublicError {
  const error = toPublicError(reason)
  return {
    code: error.code,
    message: error.message,
    recoverable: error.recoverable,
    action: error.action,
  }
}

export function reduceCaptureTranslationState(
  current: CaptureTranslationViewState,
  event: CaptureTranslationEvent,
): CaptureTranslationViewState {
  if (current.sessionId && event.sessionId !== current.sessionId) return current
  switch (event.state) {
    case 'selecting':
      return { status: 'selecting', sessionId: event.sessionId, document: null, error: null }
    case 'processing':
      return { status: 'processing', sessionId: event.sessionId, document: event.document ?? current.document, error: null }
    case 'empty':
      if (event.document) {
        return { status: 'empty', sessionId: event.sessionId, document: event.document, error: null }
      }
      return current
    case 'success':
      if (event.document) {
        return { status: 'success', sessionId: event.sessionId, document: event.document, error: null }
      }
      return current
    case 'error':
      return {
        status: 'error',
        sessionId: event.sessionId,
        document: event.document ?? current.document,
        error: event.error ?? {
          code: 'backend_failure',
          message: '截图翻译失败，请重试。',
          recoverable: true,
          action: 'retry',
        },
      }
    case 'cancelled':
      return { status: 'cancelled', sessionId: event.sessionId, document: null, error: null }
  }
}

function applyEvent(event: CaptureTranslationEvent) {
  state.value = reduceCaptureTranslationState(state.value, event)
  if (state.value.status === 'success' || state.value.status === 'empty') {
    const { srcText, tgtText } = useTranslator()
    const blocks = [...state.value.document.blocks].sort((a, b) => a.readingOrder - b.readingOrder)
    srcText.value = blocks.map(block => block.sourceText).join('\n')
    tgtText.value = blocks.map(block => block.translatedText ?? '').filter(Boolean).join('\n')
  }
}

async function ensureListener() {
  if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
    return () => {}
  }
  listening ??= captureTranslationClient.onState(applyEvent).catch(reason => {
    listening = null
    throw reason
  })
  return listening
}

export function useCaptureTranslation() {
  async function currentConfig(): Promise<TranslationConfig> {
    if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) config.value = await captureTranslationClient.getConfig()
    const { langA, langB } = useTranslator()
    return { ...config.value, sourceLanguage: langA.value, targetLanguage: langB.value }
  }

  async function startWindow(source: CaptureSourceSummary) {
    state.value = { status: 'idle', sessionId: null, document: null, error: null }
    try {
      await ensureListener()
      await captureTranslationClient.startWindow(source.sourceId, source.catalogRevision, await currentConfig())
    } catch (reason: unknown) {
      state.value = { status: 'error', sessionId: state.value.sessionId, document: state.value.document, error: publicError(reason) }
    }
  }

  async function translateRegion(revision: number) {
    if (starting.value) return
    starting.value = true
    state.value = { status: 'idle', sessionId: null, document: null, error: null }
    try {
      await ensureListener()
      await captureTranslationClient.translateRegion(revision, await currentConfig())
    } catch (reason: unknown) {
      state.value = { status: 'error', sessionId: state.value.sessionId, document: state.value.document, error: publicError(reason) }
    } finally { starting.value = false }
  }

  async function start() {
    state.value = { status: 'idle', sessionId: null, document: null, error: null }
    try {
      await ensureListener()
      const response = await captureTranslationClient.start(await currentConfig())
      if (state.value.status === 'idle') {
        state.value = {
          status: 'selecting',
          sessionId: response.sessionId,
          document: null,
          error: null,
        }
      }
    } catch (reason: unknown) {
      state.value = {
        status: 'error',
        sessionId: null,
        document: null,
        error: publicError(reason),
      }
    }
  }

  async function cancel() {
    const sessionId = state.value.sessionId
    if (!sessionId) return
    try {
      await captureTranslationClient.cancel(sessionId)
    } catch (reason: unknown) {
      state.value = {
        status: 'error',
        sessionId,
        document: state.value.document,
        error: publicError(reason),
      }
    }
  }

  function dismiss() {
    state.value = { status: 'idle', sessionId: null, document: null, error: null }
  }

  function setConfig(next: TranslationConfig) {
    config.value = { ...next }
  }

  const busy = computed(() => starting.value || state.value.status === 'selecting' || state.value.status === 'processing')

  return {
    state: readonly(state),
    config: readonly(config),
    busy,
    start,
    startWindow,
    translateRegion,
    cancel,
    dismiss,
    setConfig,
  }
}
